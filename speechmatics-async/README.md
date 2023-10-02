# speechmatics-async

**IMPORTANT: This is a work in progress, the API is subject to significant change and much of the error handling is currently lacking. We hope to eventually get this code to a production state, but for now it should serve as a guide to what a rust implementation could look like.**

This crate is based on [tokio-tungstenite](https://docs.rs/tokio-tungstenite/latest/tokio_tungstenite/), and should fit nicely into your async rust stack to allow you to run realtime transcription tasks asynchronously alongside other tasks.

## Getting Started

First things first, setting your desired feature flags. These options are:

1. realtime - enables realtime features, causes tokio and tokio-tungstenite to be installed as dependencies
2. batch - enabled batch features, causes reqwest and rand to be installed as dependencies

After installing the package via cargo, a simple use case of transcribing a file might look like this:

```rs
use speechmatics_async::realtime::*;
use std::fs::File;
use std::path::PathBuf;
use speechmatics_async::realtime::handlers::Attach;
use std::println;
use std::box::Box;
use futures::Future;
use tokio;

#[tokio::main]
fn main() {
    let mut rt_session =
        RealtimeSession::new("INSERT_KEY_HERE".to_owned(), None).unwrap();

    let test_file_path = PathBuf::new()
        .join("..")
        .join("tests")
        .join("data")
        .join("example.wav");

    let file = File::open(test_file_path).unwrap();

    let mut config: SessionConfig = Default::default();
    let audio_config = models::AudioFormat::new(models::audio_format::RHashType::File);
    config.audio_format = Some(audio_config);

    // Note that because this performs dynamic dispatch, the Future must be put inside a pinned box
    fn closure(input: models::AddTranscript) -> Pin<Box<dyn Future<Output = ()>>>  {
        Box::pin(async move {
            // This is obviously not async, but you should get the idea. You can await any async code in this block
            println!("{:?}", input)
        })
    }

    add_event_handler!(&mut rt_session, handlers::AddTranscriptHandler, closure);

    rt_session.run(config, file).await.unwrap();
}
```
