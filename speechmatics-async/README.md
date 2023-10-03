# speechmatics-async

**IMPORTANT: This is a work in progress, the API is subject to significant change and much of the error handling is currently lacking. We hope to eventually get this code to a production state, but for now it should serve as a guide to what a rust implementation could look like.**

This crate is based on [tokio-tungstenite](https://docs.rs/tokio-tungstenite/latest/tokio_tungstenite/), and should fit nicely into your async rust stack to allow you to run realtime transcription tasks asynchronously alongside other tasks.

## Getting Started

First things first, setting your desired feature flags. These options are:

1. realtime - enables realtime features, causes tokio and tokio-tungstenite to be installed as dependencies
2. batch - enabled batch features, causes reqwest and rand to be installed as dependencies

After installing the package via cargo, a simple use case of transcribing a file might look like this:

```rs
use futures::Future;
use speechmatics_async::{
    add_event_handler,
    realtime::{handlers, models, RealtimeSession, SessionConfig},
};
use std::fs::File;
use std::path::PathBuf;
use std::pin::Pin;
use tokio;

#[tokio::main]
async fn main() {
    let api_key: String = std::env::var("API_KEY").unwrap();
    let mut rt_session = RealtimeSession::new(api_key, None).unwrap();

    let test_file_path = PathBuf::new()
        .join("..")
        .join("tests")
        .join("data")
        .join("example.wav");

    let file = File::open(test_file_path).unwrap();

    let mut config: SessionConfig = Default::default();
    let audio_config = models::AudioFormat::new(models::audio_format::Type::File);
    config.audio_format = Some(audio_config);

    fn closure(input: models::AddTranscript) -> Pin<Box<dyn Future<Output = ()>>> {
        Box::pin(async move { println!("{:?}", input) })
    }

    add_event_handler!(&mut rt_session, handlers::AddTranscriptHandler, closure);

    rt_session.run(config, file).await.unwrap();
}
```
