use anyhow::Result;
use futures::{StreamExt, stream::{SplitStream, SplitSink}, pin_mut, future::{self, Either}, SinkExt, TryStreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::{tungstenite, connect_async, tungstenite::Message, WebSocketStream};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;use base64::{Engine as _, engine::general_purpose};
use url::Url;
use serde_json::from_slice;
use handlers::Attach;

use http::Request;
use std::io::Read;

use log::{error, debug, info, warn};

mod handlers;
pub mod models;

const DEFAULT_RT_URL: &str = "wss://eu2.rt.speechmatics.com/v2/en";
const DEFAULT_LANGUAGE: &str = "en";
const DEFAULT_SAMPLE_RATE: i32 = 48_000;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionConfig {
    transcription_config: Option<models::TranscriptionConfig>,
    translation_config: Option<models::TranslationConfig>,
    audio_format: Option<models::AudioFormat>,
}

enum AudioVector {
    PcmF32le(Vec<f32>),
    PcmS16le(Vec<i16>),
}

impl Default for SessionConfig {
    fn default() -> Self {
        let transcription_config: models::TranscriptionConfig = Default::default();
        let translation_config: models::TranslationConfig = Default::default();
        let audio_format: models::AudioFormat = Default::default();
        Self {
            transcription_config: Some(transcription_config),
            translation_config: Some(translation_config),
            audio_format: Some(audio_format),
        }
    }
}

type SplitStreamAlias = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

pub struct RealtimeSession {
    pub auth_token: String,
    pub rt_url: String,
    handlers: handlers::EventHandlers,
}

impl RealtimeSession {
    pub fn new(auth_token: String, rt_url: Option<String>) -> Result<Self>
    {
        let mut url = DEFAULT_RT_URL.to_owned();
        if let Some(temp_url) = rt_url {
            url = temp_url
        }
        let mut sesh = Self {
            auth_token,
            rt_url: url,
            handlers: handlers::EventHandlers::new(),
        };
        Ok(sesh)
    }

    async fn connect(&mut self) -> Result<(SenderWrapper, SplitStreamAlias)> {
        let sec_key: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        let b64 = general_purpose::STANDARD.encode(sec_key);

        let uri = Url::parse(&self.rt_url)?;
        let authority = uri.authority();
        let host = authority
            .find('@')
            .map(|idx| authority.split_at(idx + 1).1)
            .unwrap_or_else(|| authority);

        if host.is_empty() {
            todo!();
        }
        let auth_header = format!("Bearer {}", self.auth_token.clone());

        let req = Request::builder()
            .method("GET")
            .header("Host", host)
            .header("Connection", "keep-alive, Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", b64)
            .header("Authorization", auth_header)
            .uri(&self.rt_url)
            .body(())?;

        let (stream, res) = connect_async(req).await?;
        if let Some(resp) = res.body() {
            error!("failed to connect {:?}", resp);
            println!("{:?}", res);
        }

        let (writer, reader) = stream.split();
        let sender = SenderWrapper::new(writer);
        Ok((sender, reader))
    }

    async fn wait_for_start(&self, receiver: &mut SplitStreamAlias) -> Result<()> {
        let mut retries = 0;
        let max_retries = 5;
        let mut success = false;
        let value = receiver.try_for_each(move |message| async {
            let bin_data = message.into_data();
            // this deserialise will fail if not the right message type
            let _: models::RecognitionStarted = match serde_json::from_slice(&bin_data) {
                Ok(val) => {
                    val
                }
                Err(err) => {
                    warn!("Could not read value of message into RecognitionStarted struct, {:?}", err);
                    models::RecognitionStarted::new(models::recognition_started::Message::RecognitionStarted)
                }
            };
            Ok(())
        }).await?;
        Ok(())
    }

    pub async fn run<R: Read + std::marker::Send + 'static>(
        &mut self,
        config: SessionConfig,
        reader: R,
    ) -> Result<()> {
        let (mut sender, mut receiver) = self.connect().await?;
        sender.start_recognition(config).await?;
        self.wait_for_start(&mut receiver).await?;

        let send_audio = {
            sender.send_audio(reader)
        };

        let process_messages = {
            self.process_messages(receiver)
        };

        pin_mut!(process_messages, send_audio);
        match future::select(process_messages, send_audio).await {
            Either::Left(s) => s.0,
            Either::Right(s) => s.0
        }
    }

    pub async fn process_messages(&mut self, receiver: SplitStreamAlias) -> Result<()> {
        receiver.try_for_each(move |message| async {
            let data = message.into_data();
            // Parse the string of data into serde_json::Value.
            let value: models::RealtimeMessage = from_slice(&data).unwrap();
            if let Some(msg) = value.message {
                if models::Messages::EndOfTranscript == msg {
                    let _: models::EndOfTranscript = from_slice(&data).unwrap();
                };
                // self.handlers.handle_event(msg.clone(), data.clone()).unwrap();
            } else {
                error!("Something went wrong unpacking the message, the message value was None");
            };
            Ok(())
        });
        Ok(())
    }
}

pub struct ReceiverWrapper {
    pub socket: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

pub struct SenderWrapper {
    pub socket: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Message>,
    last_seq_no: i32,
}

impl SenderWrapper {
    fn new(socket: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Message>) -> Self {
        Self { socket, last_seq_no: 0 }
    }

    pub async fn send_audio<R: Read + std::marker::Send + 'static>(&mut self, mut reader: R) -> Result<()> {
        let mut buffer = vec![0u8; 8192];
        loop {
            debug!("reading audio data");
            match reader.read(&mut buffer) {
                Ok(no) => {
                    if no == 0 {
                        info!("Reader was empty, closing stream");
                        self.send_close(self.last_seq_no).await?;
                        return Ok(())
                    } else {
                        debug!("Sending audio length {no}");
                        let tu_message = Message::from(&buffer[..no]);
                        self.send_message(tu_message).await?;
                        self.last_seq_no += 1;
                    }
                }
                Err(_) => {
                    info!("encountered an error reading audio data, closing the stream");
                    self.send_close(self.last_seq_no).await?;
                }
            };
        }
    }

    async fn send_message(&mut self, message: Message) -> Result<()> {
        let mut retries = 0;
        let max_retries = 5;
        let mut success = false;
        while !success {
            match self.socket.send(message.clone()).await {
                Ok(()) => (),
                Err(err) => {
                    retries += 1;
                    if retries >= max_retries {
                        error!("{:?}", err);
                        self.socket.send(message).await?;
                        panic!("arg too many attempts to send")
                    }
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    continue;
                }
            };
            success = true
        }
        Ok(())
    }

    async fn start_recognition(&mut self, config: SessionConfig) -> Result<()> {
        let mut message: models::StartRecognition = Default::default();
        if let Some(aud) = config.audio_format {
            message.audio_format = Box::new(aud);
        }
        if let Some(transc) = config.transcription_config {
            message.transcription_config = Box::new(transc);
        }
        if let Some(transl) = config.translation_config {
            message.translation_config = Some(Box::new(transl));
        }
        let serialised_msg = serde_json::to_string(&message)?;
        let ws_message = Message::from(serialised_msg);
        self.send_message(ws_message).await
    }

    pub async fn send_close(&mut self, last_seq_no: i32) -> Result<()> {
        let message = models::EndOfStream::new(last_seq_no, models::end_of_stream::Message::EndOfStream);
        let serialised_msg = serde_json::to_string(&message)?;
        let tungstenite_msg = Message::from(serialised_msg);
        self.send_message(tungstenite_msg).await
    }

    pub async fn close(&mut self) -> Result<()> {
        self.socket.close().await?;
        Ok(())
    }
}

#[allow(unused_macros)]
macro_rules! add_event_handler {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($sesh:expr, $func_type:ty, $func:expr) => {
        let function: $func_type = $func;
        function.attach(&mut $sesh.handlers);
    };
}

#[allow(unused_imports)]
pub(crate) use add_event_handler;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::fs::File;

    use tokio;

    #[tokio::test]
    async fn test_basic_flow() {
        let mut rt_session =
            RealtimeSession::new("INSERT_API_KEY".to_owned(), None).unwrap();

        let test_file_path = PathBuf::new()
            .join("..")
            .join("tests")
            .join("data")
            .join("example.wav");

        let file = File::open(test_file_path).unwrap();

        let mut config: SessionConfig = Default::default();
        let audio_config = models::AudioFormat::new(models::audio_format::RHashType::File);
        config.audio_format = Some(audio_config);

        async fn closure(input: models::AddTranscript) -> ()  {
            println!("This is a test, you should see AddTranscript message logs in the terminal {:?}", input)
        };

        // add_event_handler!(&mut rt_session, handlers::AddTranscriptCallback, closure);

        rt_session.run(config, file).await.unwrap();
    }
}
