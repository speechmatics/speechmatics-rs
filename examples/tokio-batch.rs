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
        .join(".")
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

    // now delete the job
    let delete_res = batch_client
        .delete_job(&job_res.id, Some(true))
        .await
        .unwrap();
    assert!(delete_res.job.status == models::job_details::Status::Deleted);
}
