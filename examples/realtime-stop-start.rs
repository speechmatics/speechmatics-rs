use speechmatics::realtime::*;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::{self, fs::File, io::AsyncReadExt, try_join};
use tokio::sync::mpsc;


#[tokio::main]
async fn main() {
    let api_key: String = std::env::var("API_KEY").unwrap();
    let (mut rt_session, mut receive_channel) = RealtimeSession::new(api_key, None).unwrap();


    let mut config: SessionConfig = Default::default();
    let audio_config = models::AudioFormat::new(models::audio_format::Type::File);
    config.audio_format = Some(audio_config);

    let message_task = tokio::spawn(async move {
        while let Some(message) = receive_channel.recv().await {
            match message {
                ReadMessage::AddTranscript(mess) => {
                    print!("{:?}", mess)
                }
                ReadMessage::EndOfTranscript(_) => return,
                _ => {}
            }
        }
    });

    let (sender, receiver) = mpsc::channel::<u8>(16000);

    let run_task = { rt_session.run(config, receiver) };

    let reader_task = async {
      tokio::time::sleep(std::time::Duration::from_secs(20)).await;
      let test_file_path = PathBuf::new()
        .join(".")
        .join("tests")
        .join("data")
        .join("example.wav");

      let file_buffer = &mut [0; 16000];
      let file = File::open(test_file_path).await.unwrap();
      loop {
        file.read(file_buffer);
        file_buffer.into_iter().for_each(move |x| sender.blocking_send(*x).expect("failed to send"));
      }
    };

    try_join!(
        async move { message_task.await.map_err(anyhow::Error::from) },
        run_task
    )
    .unwrap();

}
