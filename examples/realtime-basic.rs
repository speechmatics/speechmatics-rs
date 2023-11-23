use futures::Future;
use speechmatics::{
    add_event_handler,
    realtime::{handlers, models, RealtimeSession, SessionConfig},
};
use std::path::PathBuf;
use std::pin::Pin;
use tokio::{self, fs::File};

#[tokio::main]
async fn main() {
    let api_key: String = std::env::var("API_KEY").unwrap();
    let mut rt_session = RealtimeSession::new(api_key, None).unwrap();

    let test_file_path = PathBuf::new()
        .join(".")
        .join("tests")
        .join("data")
        .join("example.wav");

    let file = File::open(test_file_path).await.unwrap();

    let mut config: SessionConfig = Default::default();
    let audio_config = models::AudioFormat::new(models::audio_format::Type::File);
    config.audio_format = Some(audio_config);

    fn closure(input: models::AddTranscript) -> Pin<Box<dyn Future<Output = ()>>> {
        Box::pin(async move { println!("{:?}", input) })
    }

    add_event_handler!(&mut rt_session, handlers::AddTranscriptHandler, closure);

    rt_session.run(config, file).await.unwrap();
}
