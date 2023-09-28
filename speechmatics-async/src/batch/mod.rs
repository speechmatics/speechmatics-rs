use anyhow::Result;
use models::*;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    multipart::{Form, Part},
    Client,
};
use std::fs;
use url::Url;

pub mod models;

pub const DEFAULT_BATCH_URL: &str = "https://asr.api.speechmatics.com/v2/";
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct BatchClient {
    batch_url: Url,
    client: Client,
    default_query: Vec<(String, String)>,
}

impl BatchClient {
    pub fn new(api_key: &str, batch_url: Option<url::Url>) -> Result<Self> {
        let mut headers = HeaderMap::new();

        let auth_header = format!("Bearer {}", api_key);
        headers.append(AUTHORIZATION, HeaderValue::from_str(&auth_header)?);

        let client = Client::builder().default_headers(headers).build()?;
        let mut set_url = Url::parse(DEFAULT_BATCH_URL)?;
        if let Some(batch_url_set) = batch_url {
            set_url = batch_url_set
        }

        let default_query = vec![("sm-sdk".to_owned(), format!("rs-{}", VERSION))];

        Ok(Self {
            batch_url: set_url,
            client,
            default_query,
        })
    }

    pub async fn submit_job(
        &self,
        config: JobConfig,
        file_path: std::path::PathBuf,
    ) -> Result<CreateJobResponse> {
        let url = self.batch_url.join("jobs")?;
        let file = fs::read(&file_path).unwrap();
        let config_text = serde_json::to_string(&config)?;

        let mut file_name: String = "".to_owned();
        if let Some(fil_name) = file_path.file_name() {
            if let Some(string_file) = fil_name.to_str() {
                file_name = string_file.to_owned();
            }
        }

        let some_file = Part::stream(file).file_name(file_name);

        let form = Form::new()
            .part("data_file", some_file)
            .text("config", config_text);

        let res = self.client.post(url).multipart(form).send().await;
        let result = res?.error_for_status()?.bytes().await?;

        let serde_res = serde_json::from_slice::<CreateJobResponse>(&result)?;
        Ok(serde_res)
    }

    pub async fn get_job(&self, job_id: &str) -> Result<RetrieveJobResponse> {
        let url = self.batch_url.join("jobs/")?.join(job_id)?;

        let res = self.client.get(url).send().await;
        let result = res?.error_for_status()?.bytes().await?;

        let serde_res = serde_json::from_slice::<RetrieveJobResponse>(&result)?;
        Ok(serde_res)
    }

    pub async fn get_jobs(
        &self,
        limit: Option<i32>,
        include_deleted: Option<bool>,
    ) -> Result<RetrieveJobsResponse> {
        let url = self.batch_url.join("jobs")?;

        let mut queries = self.default_query.clone();

        if let Some(lim) = limit {
            queries.push(("limit".to_owned(), format!("{}", lim)))
        }

        if let Some(del) = include_deleted {
            queries.push(("include_deleted".to_owned(), format!("{}", del)))
        }

        let res = self.client.get(url).query(&queries).send().await;
        let result = res?.error_for_status()?.bytes().await?;

        let serde_res = serde_json::from_slice::<RetrieveJobsResponse>(&result)?;
        Ok(serde_res)
    }

    pub async fn get_result(
        &self,
        job_id: &str,
        format: Option<&str>,
    ) -> Result<RetrieveTranscriptResponse> {
        let url = self
            .batch_url
            .join("jobs/")?
            .join(&format!("{}/", job_id))?
            .join("transcript")?;

        let mut queries = self.default_query.clone();

        if let Some(form) = format {
            queries.push(("format".to_owned(), form.to_owned()))
        }

        let res = self.client.get(url).query(&queries).send().await;
        let result = res?.error_for_status()?.bytes().await?;

        let serde_res = serde_json::from_slice::<RetrieveTranscriptResponse>(&result)?;
        Ok(serde_res)
    }

    pub async fn delete_job(&self, job_id: &str, force: Option<bool>) -> Result<DeleteJobResponse> {
        let url = self.batch_url.join("jobs/")?.join(job_id)?;

        let mut queries = self.default_query.clone();

        if let Some(force_set) = force {
            queries.push(("force".to_owned(), format!("{}", force_set)))
        }

        let res = self.client.delete(url).query(&queries).send().await;
        let result = res?.error_for_status()?.bytes().await?;

        let serde_res = serde_json::from_slice::<DeleteJobResponse>(&result)?;
        Ok(serde_res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;


    async fn submit_job_util(batch_client: &BatchClient) -> Result<CreateJobResponse> {
        let test_file_path = PathBuf::new()
            .join("..")
            .join("tests")
            .join("data")
            .join("example.wav");

        let mut config = JobConfig::default();
        let mut transcription_config = TranscriptionConfig::default();
        transcription_config.language = "en".to_owned();
        config.transcription_config = Some(Box::new(transcription_config));

        let job_res = batch_client.submit_job(config, test_file_path).await;
        job_res
    }

    #[tokio::test]
    async fn test_not_authorised() {
        let batch_client = BatchClient::new("blah", None).unwrap();

        let job_res = submit_job_util(&batch_client).await;
        match job_res {
            Ok(_) => assert!(false),
            Err(err) => {
                assert!(err.is::<reqwest::Error>())
            }
        }
    }

    #[tokio::test]
    async fn test_submit_job_success() {
        let api_key: String = std::env::var("SM_API_KEY").unwrap();
        let batch_client = BatchClient::new(&api_key, None).unwrap();

        let job_res = submit_job_util(&batch_client).await.unwrap();
        assert!(job_res.id != "")
    }

    #[tokio::test]
    async fn test_get_job() {
        let api_key: String = std::env::var("SM_API_KEY").unwrap();
        let batch_client = BatchClient::new(&api_key, None).unwrap();

        let job_res = submit_job_util(&batch_client).await.unwrap();
        let get_job_res = batch_client.get_job(&job_res.id).await.unwrap();
        assert!(get_job_res.job.data_name != "");
        if let Some(dur) = get_job_res.job.duration {
            assert!(dur > 0);
        } else {
            assert!(false)
        }
    }

    #[tokio::test]
    async fn test_get_jobs() {
        let api_key: String = std::env::var("SM_API_KEY").unwrap();
        let batch_client = BatchClient::new(&api_key, None).unwrap();

        let _ = submit_job_util(&batch_client).await.unwrap();
        let _ = submit_job_util(&batch_client).await.unwrap();
        let _ = submit_job_util(&batch_client).await.unwrap();
        let job_res = batch_client.get_jobs(Some(2), None).await.unwrap();
        assert!(job_res.jobs.len() == 2)
    }

    #[tokio::test]
    async fn test_get_result() {
        let api_key: String = std::env::var("SM_API_KEY").unwrap();
        let batch_client = BatchClient::new(&api_key, None).unwrap();

        let job_res = submit_job_util(&batch_client).await.unwrap();
        let get_result_res = batch_client
            .get_result(&job_res.id, Some("json-v2"))
            .await
            .unwrap();
        assert!(get_result_res.format == "json-v2");
        assert!(get_result_res.results.len() != 0)
    }

    #[tokio::test]
    async fn test_delete_job() {
        let api_key: String = std::env::var("SM_API_KEY").unwrap();
        let batch_client = BatchClient::new(&api_key, None).unwrap();

        let job_res = submit_job_util(&batch_client).await.unwrap();
        let delete_res = batch_client
            .delete_job(&job_res.id, Some(true))
            .await
            .unwrap();
        assert!(delete_res.job.status == models::job_details::Status::Deleted);
    }
}
