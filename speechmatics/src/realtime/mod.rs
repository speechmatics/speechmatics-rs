use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use http::Request;

#[cfg(not(test))]
use log::{debug, error, warn, info};
use tungstenite::protocol::WebSocketConfig;

#[cfg(test)]
use std::{println as debug, println as error, println as warn, println as info};
use std::io::Read;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use url::Url;
use serde_json::from_slice;


pub mod handlers;
pub mod models;

const DEFAULT_RT_URL: &str = "wss://neu.rt.speechmatics.com/v2/en";
const DEFAULT_LANGUAGE: &str = "en";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionConfig {
    transcription_config: Option<models::TranscriptionConfig>,
    translation_config: Option<models::TranslationConfig>,
    audio_format: Option<models::AudioFormat>,
}

impl Default for SessionConfig {
    fn default() -> Self {
        let mut transcription_config: models::TranscriptionConfig = Default::default();
        transcription_config.language = DEFAULT_LANGUAGE.to_owned();
        transcription_config.max_delay = Some(3.0);
        let audio_format: models::AudioFormat = Default::default();
        Self {
            transcription_config: Some(transcription_config),
            translation_config: None,
            audio_format: Some(audio_format),
        }
    }
}

pub struct RealtimeSession {
    pub auth_token: String,
    pub rt_url: String,
    handlers: handlers::EventHandlers,
    reader_empty: bool,
}

impl RealtimeSession {
    pub fn new(auth_token: String, rt_url: Option<String>) -> Self {
        let mut url = DEFAULT_RT_URL.to_owned();
        if let Some(temp_url) = rt_url {
            url = temp_url
        }
        Self {
            auth_token,
            rt_url: url,
            handlers: handlers::EventHandlers::new(),
            reader_empty: false,
        }
    }

    fn connect(&mut self) -> Result<SocketWrapper> {
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

        println!("{auth_header}");

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

        let (mut stream, res) = connect(req)?;
        if let Some(resp) = res.body() {
            error!("failed to connect {:?}", resp);
            println!("{:?}", res);
        }
        let s = stream.get_mut();
        match s {
            tungstenite::stream::MaybeTlsStream::Plain(s) => s.set_nonblocking(true)?,
            tungstenite::stream::MaybeTlsStream::NativeTls(s) => {
                s.get_mut().set_nonblocking(true)?;
            }
            x => panic!("Received unknown stream type: {:?}", x),
        }

        let mut ws_conf = WebSocketConfig::default();
        ws_conf.max_message_size = None;
        stream.set_config(|conf| *conf = ws_conf.clone());
        Ok(SocketWrapper::new(stream))
    }

    fn wait_for_start(&self, wrapper: &mut SocketWrapper) -> Result<()> {
        let mut success = false;
        let mut retries = 0;
        let max_retries = 5;
        while !success {
            if let Some(message) = wrapper.read()? {
                println!("{message}");
                let bin_data = message.into_data();
                // this deserialise will fail if not the right message type
                let _: models::RecognitionStarted = match serde_json::from_slice(&bin_data) {
                    Ok(val) => {
                        success = true;
                        val
                    }
                    Err(err) => {
                        warn!("Could not read value of message into RecognitionStarted struct, {:?}", err);
                        models::RecognitionStarted::new(models::recognition_started::Message::RecognitionStarted)
                    }
                };
            } else {
                if retries >= max_retries {
                    // return Err(anyhow::Error::new(tungstenite::Error::));
                };
                retries += 1;
                if !success {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            }
        }
        Ok(())
    }

    pub fn run<R: Read + std::marker::Send + 'static>(
        &mut self,
        config: SessionConfig,
        mut reader: R,
    ) -> Result<()> {
        let mut wrapper = self.connect()?;
        wrapper.start_recognition(config)?;
        self.wait_for_start(&mut wrapper)?;
        let mut buffer = vec![0u8; 8192];

        loop {
            // If there is a message, read it, otherwise send a chunk of audio data and check messages again
            if let Some(message) = wrapper.read()? {
                debug!("{message}");
                let bin_data = message.into_data();
                // Parse the string of data into serde_json::Value.
                let value: models::RealtimeMessage = from_slice(&bin_data)?;
                if let Some(msg) = value.message {
                    if models::Messages::EndOfTranscript == msg {
                        let _: models::EndOfTranscript = from_slice(&bin_data)?;
                        wrapper.close()?;
                        break;
                    };
                    self.handlers.handle_event(msg, bin_data)?;
                } else {
                    error!("Something went wrong unpacking the message, the message value was None");
                }
            };
            if !self.reader_empty {
                match reader.read(&mut buffer) {
                    Ok(no) => {
                        if no == 0 {
                            info!("Reader was empty, closing stream");
                            wrapper.send_close()?;
                            self.reader_empty = true;
                        } else {
                            debug!("Sending audio length {no}");
                            let tu_message = Message::from(&buffer[..no]);
                            wrapper.send_message(tu_message)?;
                            wrapper.last_seq_no += 1;
                        }
                    }
                    Err(_) => {
                        info!("encountered an error reading audio data, closing the stream");
                        wrapper.send_close()?;
                        self.reader_empty = true;
                    }
                };
            }
        }
        Ok(())
    }
}

pub struct SocketWrapper {
    pub socket: WebSocket<MaybeTlsStream<TcpStream>>,
    last_seq_no: i32,
}

impl SocketWrapper {
    fn new(socket: WebSocket<MaybeTlsStream<TcpStream>>) -> Self {
        Self { socket, last_seq_no: 0 }
    }

    fn send_message(&mut self, message: Message) -> Result<()> {
        let mut retries = 0;
        let max_retries = 5;
        let mut success = false;
        while !success {
            match self.socket.write_message(message.clone()) {
                Ok(()) => (),
                Err(err) => {
                    retries += 1;
                    if retries >= max_retries {
                        error!("{:?}", err);
                        self.socket.write_message(message)?;
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

    fn start_recognition(&mut self, config: SessionConfig) -> Result<()> {
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
        self.send_message(ws_message)
    }

    pub fn send_close(&mut self) -> Result<()> {
        let message = models::EndOfStream::new(self.last_seq_no, models::end_of_stream::Message::EndOfStream);
        let serialised_msg = serde_json::to_string(&message)?;
        let tungstenite_msg = Message::from(serialised_msg);
        self.send_message(tungstenite_msg)
    }

    pub fn read(&mut self) -> Result<Option<Message>> {
        let mess = match self.socket.read_message() {
            Ok(mess) => Some(mess),
            Err(e) => {
                if let tungstenite::error::Error::Io(e) = &e {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        return Ok(None)
                    }
                }
                error!("{:?}", e);
                return Err(anyhow::Error::from(e))
            }
        };
        Ok(mess)
    }

    pub fn close(&mut self) -> Result<()> {
        self.socket.close(None)?;
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
    use std::fs::File;
    use std::path::PathBuf;
    use crate::realtime::handlers::Attach;

    #[test]
    fn test_basic_flow() {
        let mut rt_session =
            RealtimeSession::new("INSERT_API_KEY".to_owned(), None);

        let test_file_path = PathBuf::new()
            .join("..")
            .join("tests")
            .join("data")
            .join("example.wav");

        let file = File::open(test_file_path).unwrap();

        let mut config: SessionConfig = Default::default();
        let audio_config = models::AudioFormat::new(models::audio_format::RHashType::File);
        config.audio_format = Some(audio_config);

        let closure: fn(models::AddTranscript) ->  () = |message: models::AddTranscript| {
            println!("This is a test, you should see AddTranscript message logs in the terminal {:?}", message)
        };

        add_event_handler!(&mut rt_session, handlers::AddTranscriptCallback, closure);

        rt_session.run(config, file).unwrap();
    }
}
