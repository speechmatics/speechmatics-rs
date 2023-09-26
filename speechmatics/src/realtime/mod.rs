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
    socket: SocketWrapper,
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
            socket: SocketWrapper::new(),
        }
    }

    fn connect(&mut self) -> Result<()> {
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
        ws_conf.write_buffer_size = 1000_000;
        ws_conf.max_write_buffer_size = 1000_001;
        stream.set_config(|conf| *conf = ws_conf.clone());
        self.socket.socket = Some(Arc::new(Mutex::new(stream)));
        Ok(())
    }

    fn wait_for_start(&self) -> Result<()> {
        let mut success = false;
        let mut retries = 0;
        let max_retries = 5;
        while !success {
            if let Some(message) = self.socket.read()? {
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
        self.connect()?;
        self.socket.start_recognition(config)?;
        self.wait_for_start()?;
        let mut buffer = vec![0u8; 8192];

        loop {
            // If there is a message, read it, otherwise send a chunk of audio data and check messages again
            if let Some(message) = self.socket.read()? {
                debug!("{message}");
                let bin_data = message.into_data();
                // Parse the string of data into serde_json::Value.
                let value: models::RealtimeMessage = from_slice(&bin_data)?;
                if let Some(msg) = value.message {
                    if models::Messages::EndOfTranscript == msg {
                        let _: models::EndOfTranscript = from_slice(&bin_data)?;
                        self.socket.close()?;
                        break;
                    };
                    self.handlers.handle_event(msg, bin_data)?;
                } else {
                    error!("Something went wrong unpacking the message, the message value was None");
                }
            };
            debug!("reading audio data");
            match reader.read(&mut buffer) {
                Ok(no) => {
                    if no == 0 {
                        info!("Reader was empty, closing stream");
                        self.socket.send_close(self.socket.last_seq_no)?;
                        return Ok(())
                    } else {
                        debug!("Sending audio length {no}");
                        let tu_message = Message::from(&buffer[..no]);
                        self.socket.send_message(tu_message)?;
                        self.socket.last_seq_no += 1;
                    }
                }
                Err(_) => {
                    info!("encountered an error reading audio data, closing the stream");
                    self.socket.send_close(self.socket.last_seq_no)?;
                }
            };
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct SocketWrapper {
    pub socket: Option<Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>>,
    last_seq_no: i32,
}

impl SocketWrapper {
    fn new() -> Self {
        Self { socket: None, last_seq_no: 0 }
    }

    fn send_message(&self, message: Message) -> Result<()> {
        if let Some(ws_stream) = &self.socket {
            let mut stream = match ws_stream.lock() {
                Ok(s) => s,
                Err(fail_lock) => {
                    let guard = fail_lock.into_inner();
                    guard
                }
            };
            let mut retries = 0;
            let max_retries = 5;
            let mut success = false;
            while !success {
                match stream.send(message.clone()) {
                    Ok(()) => (),
                    Err(err) => {
                        retries += 1;
                        if retries >= max_retries {
                            error!("{:?}", err);
                            stream.send(message)?;
                            panic!("arg too many attempts to send")
                        }
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        continue;
                    }
                };
                success = true
            }
        }
        Ok(())
    }

    fn start_recognition(&self, config: SessionConfig) -> Result<()> {
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

    pub fn send_close(&self, last_seq_no: i32) -> Result<()> {
        let message = models::EndOfStream::new(last_seq_no, models::end_of_stream::Message::EndOfStream);
        let serialised_msg = serde_json::to_string(&message)?;
        let tungstenite_msg = Message::from(serialised_msg);
        self.send_message(tungstenite_msg)
    }

    pub fn can_read(&self) -> Result<bool> {
        if let Some(sock) = &self.socket {
            let stream = match sock.lock() {
                Ok(s) => s,
                Err(fail_lock) => {
                    let guard = fail_lock.into_inner();
                    guard
                }
            };
            Ok(stream.can_read())
        } else {
            Ok(false)
        }
    }

    pub fn can_write(&self) -> Result<bool> {
        if let Some(sock) = &self.socket {
            let stream = match sock.lock() {
                Ok(s) => s,
                Err(fail_lock) => {
                    let guard = fail_lock.into_inner();
                    guard
                }
            };
            Ok(stream.can_write())
        } else {
            Ok(false)
        }
    }

    pub fn read(&self) -> Result<Option<Message>> {
        if let Some(sock) = &self.socket {
            let mut stream = match sock.lock() {
                Ok(s) => s,
                Err(fail_lock) => {
                    let guard = fail_lock.into_inner();
                    guard
                }
            };
            let mess = match stream.read() {
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
        } else {
            todo!()
        }
    }

    pub fn close(&self) -> Result<()> {
        if let Some(sock) = &self.socket {
            let mut lock_sock = sock.lock().unwrap();
            lock_sock.close(None)?;
            Ok(())
        } else {
            todo!()
        }
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
