# Speechmatics Rust SDK

**IMPORTANT: This is a work in progress, the API is subject to significant change and much of the error handling is currently lacking. We hope to eventually get this code to a production state, but for now it should serve as a guide to what a rust implementation could look like. We welcome contributions, so please don't hesitate to reach out!**

This crate uses [tokio-tungstenite](https://docs.rs/tokio-tungstenite/latest/tokio_tungstenite/) for realtime and [reqwest](https://docs.rs/reqwest/latest/reqwest/), and should fit nicely into your async rust stack to allow you to run transcription tasks asynchronously alongside other tasks.

## Getting Started

First things first, setting your desired feature flags. These options are:

1. realtime - enables realtime features, causes tokio and tokio-tungstenite to be installed as dependencies
2. batch - enabled batch features, causes reqwest and rand to be installed as dependencies

In order to connect to the API, you will also need an API key. You can get a key from our [portal](https://portal.speechmatics.com/manage-access/). You'll need to create a free account to access the portal (no credit card required).

## Transcribing In Realtime

To transcribe in realtime, you'll need to install the futures, tokio and speechmatics crates. Then you can run the following code in your main.rs file.  Don't forget to update the API key and file path.

```rs
use futures::Future;
use speechmatics::{
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

## Transcribing With Batch

To transcribe in batch, you'll need to install the an async runtime of your choice. Here, we're using tokio for consistency's sake. Then the following code can be added to your main.rs file and run. Don't forget to update the API key and file path.

```rs
use speechmatics::batch::{
    models::{self, JobConfig, TranscriptionConfig},
    BatchClient,
};
use std::path::PathBuf;
use tokio;

#[tokio::main]
async fn main() {
    // instantiate the client
    let api_key: String = std::env::var("API_KEY").unwrap();
    let batch_client = BatchClient::new(&api_key, None).unwrap();

    // set up the path to the file and load in the config
    let test_file_path = PathBuf::new()
        .join("..")
        .join("tests")
        .join("data")
        .join("example.wav");

    let mut config = JobConfig::default();
    let mut transcription_config = TranscriptionConfig::default();
    transcription_config.language = "en".to_owned();
    config.transcription_config = Some(Box::new(transcription_config));

    // submit the job
    let job_res = batch_client
        .submit_job(config, test_file_path)
        .await
        .unwrap();

    // wait for the job to return a completed status, or to enter an error status in which case panic
    let mut success = false;
    let mut retries = 0;
    while !success {
        let get_job_res = batch_client.get_job(&job_res.id).await.unwrap();
        if get_job_res.job.status == models::job_details::Status::Done {
            success = true
        } else if get_job_res.job.status != models::job_details::Status::Running {
            panic!("Job failed");
        } else {
            if retries > 6 {
                panic!("Job took too long to complete");
            }
            retries += 1;
            std::thread::sleep(std::time::Duration::from_millis(3000));
        }
    }

    // get the json transcript of the job
    let get_result_res = batch_client.get_json_result(&job_res.id).await.unwrap();
    println!("{:?}", get_result_res);
}
```

## Examples

You can find more examples of the code in the [examples folder](./examples/). In order to run the examples, you will need to set the API_KEY environment variable. This should be your API key for the speechmatics API.

## Getting Help

If you need help, there are a few channels you can use:

- Raise an issue on our [issues board](https://github.com/speechmatics/speechmatics-rs/issues)
- Reach out to us on our [discussions forum](https://github.com/orgs/speechmatics/discussions)
- Ask a question on [Stack Overflow](https://stackoverflow.com/)

## Contributing

If you are interested in contributing, please refer to [CONTRIBUTING](./contributing.md)

## Supported Rust Version

This crate has been built with rust version 1.72.0. Although it may well work on older versions of rust, we do not guarantee it.

## License

This project is licensed under MIT.
