# speechmatics (sync)

**IMPORTANT: This is a work in progress, the API is subject to significant change and much of the error handling is currently lacking. We hope to eventually get this code to a production state, but for now it should serve as a guide to what a rust implementation could look like.**

This crate is a simple, 100% synchronous implementation of our Realtime API for the sort of refined individual who likes their binary as small as possible. It is built around [tungstenite-rs](https://docs.rs/tungstenite/latest/tungstenite/), and has a relatively small number of dependencies (at least compared to the async package).

## Getting Started

After installing the package via cargo, a simple use case of transcribing a file might look like this:

```rs
use speechmatics::realtime::*;
use std::fs::File;
use std::path::PathBuf;
use speechmatics::realtime::handlers::Attach;
use std::println;

fn main() {
    // Open a file for transcribing
    let test_file_path = PathBuf::new()
        .join("..")
        .join("tests")
        .join("data")
        .join("example.wav");
    let file = File::open(test_file_path).unwrap();

    let mut rt_session =
        RealtimeSession::new("INSERT_API_KEY".to_owned(), None);
    rt_session.connect().unwrap();

    // Set config (we want to set type to file, not raw)
    let mut config: SessionConfig = Default::default();
    let audio_config = models::AudioFormat::new(models::audio_format::RHashType::File);
    config.audio_format = Some(audio_config);

    let closure: fn(models::AddTranscript) ->  () = |message: models::AddTranscript| {
        println!("This is a test, you should see AddTranscript message logs in the terminal {:?}", message)
    };

    add_event_handler!(&mut rt_session, handlers::AddTranscriptCallback, closure);

    rt_session.run(config, file).unwrap();
}
```

Note that since this is single threaded and synchronous, it will block the main execution loop for code. Of course, there is nothing stopping you from running this in a separate thread, or spinning up multiple threads to handling multiple transcription sessions.