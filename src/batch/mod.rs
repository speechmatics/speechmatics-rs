//! The main entry point for the batch jobs API. Provides a struct which wraps a client and comes with associated API methods.

use anyhow::Result;
use models::*;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    multipart::{Form, Part},
    Client,
};
use std::fs;
use url::Url;

#[allow(missing_docs)]
pub mod models;

/// The default URL for the batch runtime.
///
/// This is the standard URL for self-service customers, and some enterprise customers.
/// Some customers may wish instead to access other European, American or Australian environments.
/// A full list of URLs can be found in our [docs](https://docs.speechmatics.com/introduction/authentication#supported-endpoints).
pub const DEFAULT_BATCH_URL: &str = "https://asr.api.speechmatics.com/v2/";
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// BatchClient - batch client is the main wrapper for making batch requests.
/// It holds the url in question along with the client object.
/// None of its properties are public.
pub struct BatchClient {
    batch_url: Url,
    client: Client,
    default_query: Vec<(String, String)>,
}

impl BatchClient {
    /// Instantiates a new batch client. Parses the provided URL or use the defaults
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

    /// Submits a job to the batch jobs API based on a path to a file.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use std::box::Box;
    /// use speechmatics::batch::{
    ///     BatchClient,
    ///     models::{JobConfig, TranscriptionConfig}
    /// };
    ///
    /// let batch_client = BatchClient::new("API_KEY", None).unwrap();
    ///
    /// let test_file_path = PathBuf::new()
    ///     .join("..")
    ///     .join("tests")
    ///     .join("data")
    ///     .join("example.wav");
    ///
    /// let mut config = JobConfig::default();
    /// let mut transcription_config = TranscriptionConfig::default();
    /// transcription_config.language = "en".to_owned();
    /// config.transcription_config = Some(Box::new(transcription_config));
    ///
    /// let job_res = batch_client.submit_job(config, test_file_path).await.unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// The following error states are possible:
    ///     - If the file can't be read (e.g. it doesn't exist)
    ///     - If there is an issue converting the file path to a file name string
    ///     - If there is an error in the API, which could be any standard HTTP error code
    ///     - If the response cannot be parsed from bytes into the correct struct
    ///
    pub async fn submit_job(
        &self,
        config: JobConfig,
        file_path: std::path::PathBuf,
    ) -> Result<CreateJobResponse> {
        let url = self.batch_url.join("jobs")?;
        let file = fs::read(&file_path)?;
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

    /// Get details for a batch job. This includes the job config, metadata and status, but does NOT include the result.
    ///
    /// # Example
    ///
    /// ```
    /// use speechmatics::batch::BatchClient;
    ///
    /// let batch_client = BatchClient::new("API_KEY", None).unwrap();
    /// let get_job_res = batch_client.get_job("JOB_ID").await.unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// This function can error with the usual HTTP status code errors.
    /// It will also error if it fails to parse the output for whatever reason.
    ///
    pub async fn get_job(&self, job_id: &str) -> Result<RetrieveJobResponse> {
        let url = self.batch_url.join("jobs/")?.join(job_id)?;

        let res = self.client.get(url).send().await;
        let result = res?.error_for_status()?.bytes().await?;

        let serde_res = serde_json::from_slice::<RetrieveJobResponse>(&result)?;
        Ok(serde_res)
    }

    /// Get a list of jobs. This includes the job config, redacted metadata and status, but does NOT include the result.
    ///
    /// Setting the limit allows controlling the number of jobs returned.
    /// If no limit is set, the jobs' blob data (i.e. audio) will also be returned.
    ///
    /// Setting include_deleted determines whether to return jobs with a status of deleted. The default is false.
    /// Deleted jobs have most of their metadata wiped.
    ///
    /// Setting created_before sets the date as a cursor. This allows searching results in a paginated way.
    /// This only works in conjunction with the limit parameter.
    ///
    /// # Example
    ///
    /// ```
    /// use speechmatics::batch::BatchClient;
    ///
    /// let batch_client = BatchClient::new("API_KEY", None).unwrap();
    /// let jobs = batch_client.get_jobs(Some(5), Some(true)).await.unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// This function can error with the usual HTTP status code errors.
    /// It will also error if it fails to parse the server response for whatever reason.
    ///
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

    /// Gets the json-formatted result of a batch job.
    /// This will include all the requested results (e.g. transcript, translation, summary) as well as config and metadata.
    ///
    /// # Example
    ///
    /// ```
    /// use std::println;
    /// use speechmatics::batch::BatchClient;
    ///
    /// let batch_client = BatchClient::new("API_KEY", None).unwrap();
    /// let get_result_res = batch_client.get_json_result("JOB_ID").await.unwrap();
    /// println!("{:?}", get_result_res);
    /// ```
    ///
    /// # Errors
    ///
    /// This function can error with the usual HTTP status code errors.
    /// It will also error if it fails to parse the server response for whatever reason.
    ///
    /// A common failure state occurs when requesting a result for an incomplete job.
    /// To this end, you should implement polling based on the get_job method
    /// to check the job status of a recently submitted job.
    ///
    pub async fn get_json_result(&self, job_id: &str) -> Result<RetrieveTranscriptResponse> {
        let url = self
            .batch_url
            .join("jobs/")?
            .join(&format!("{}/", job_id))?
            .join("transcript")?;

        let mut queries = self.default_query.clone();

        queries.push(("format".to_owned(), "json-v2".to_owned()));

        let res = self.client.get(url).query(&queries).send().await;
        let result = res?.error_for_status()?.bytes().await?;

        let serde_res = serde_json::from_slice::<RetrieveTranscriptResponse>(&result)?;
        Ok(serde_res)
    }

    /// Gets the text result of a batch job.
    /// This will only include the transcript, without any meta data, translations or summary.
    ///
    /// # Example
    ///
    /// ```
    /// use std::println;
    /// use speechmatics::batch::BatchClient;
    ///
    /// let batch_client = BatchClient::new("API_KEY", None).unwrap();
    /// let get_result_res = batch_client.get_text_result("JOB_ID").await.unwrap();
    /// println!("{:?}", get_result_res);
    /// ```
    ///
    /// # Errors
    ///
    /// This function can error with the usual HTTP status code errors.
    /// It will also error if it fails to parse the server response for whatever reason.
    ///
    /// A common failure state occurs when requesting a result for an incomplete job.
    /// To this end, you should implement polling based on the get_job method
    /// to check the job status of a recently submitted job.
    ///
    pub async fn get_text_result(&self, job_id: &str) -> Result<String> {
        let url = self
            .batch_url
            .join("jobs/")?
            .join(&format!("{}/", job_id))?
            .join("transcript")?;

        let mut queries = self.default_query.clone();

        queries.push(("format".to_owned(), "txt".to_owned()));

        let res = self.client.get(url).query(&queries).send().await;
        let result = res?.error_for_status()?.bytes().await?;

        let serde_res = String::from_utf8(result.to_vec())?;
        Ok(serde_res)
    }

    /// Gets the SRT result of a batch job. This will be returned as a String.
    /// This will only include the transcript, without any meta data, translations or summary.
    ///
    /// # Example
    ///
    /// ```
    /// use std::println;
    /// use speechmatics::batch::BatchClient;
    ///
    /// let batch_client = BatchClient::new("API_KEY", None).unwrap();
    /// let get_result_res = batch_client.get_text_result("JOB_ID").await.unwrap();
    /// println!("{:?}", get_result_res);
    /// ```
    ///
    /// # Errors
    ///
    /// This function can error with the usual HTTP status code errors.
    /// It will also error if it fails to parse the server response for whatever reason.
    ///
    /// A common failure state occurs when requesting a result for an incomplete job.
    /// To this end, you should implement polling based on the get_job method
    /// to check the job status of a recently submitted job.
    ///
    pub async fn get_srt_result(&self, job_id: &str) -> Result<String> {
        let url = self
            .batch_url
            .join("jobs/")?
            .join(&format!("{}/", job_id))?
            .join("transcript")?;

        let mut queries = self.default_query.clone();

        queries.push(("format".to_owned(), "srt".to_owned()));

        let res = self.client.get(url).query(&queries).send().await;
        let result = res?.error_for_status()?.bytes().await?;

        let serde_res = serde_json::from_slice::<String>(&result)?;
        Ok(serde_res)
    }

    /// Delete a given job.
    ///
    /// Incomplete jobs cannot normally be deleted.
    /// If the optional force parameter is provided, then the job will be deleted even if it isn't yet completed.
    ///
    /// # Example
    ///
    /// ```
    /// use std::println;
    /// use speechmatics::batch::BatchClient;
    ///
    /// let batch_client = BatchClient::new("API_KEY", None).unwrap();
    /// let get_result_res = batch_client.delete_job("JOB_ID", Some(true)).await.unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// This function can error with the usual HTTP status code errors.
    /// It will also error if it fails to parse the server response for whatever reason.
    ///
    /// A common failure state occurs when trying to delete a job currently being processed.
    /// To avoid this, either poll for the job status, or set the optional force parameter to true.
    ///
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
            .join(".")
            .join("tests")
            .join("data")
            .join("example.wav");

        let mut config = JobConfig::default();
        let mut transcription_config = TranscriptionConfig::default();
        transcription_config.language = "en".to_owned();
        config.transcription_config = Some(Box::new(transcription_config));

        batch_client.submit_job(config, test_file_path).await
    }

    #[tokio::test]
    async fn test_not_authorised() {
        let batch_client = BatchClient::new("blah", None).unwrap();

        let job_res = submit_job_util(&batch_client).await;
        match job_res {
            Ok(_) => panic!("Something went wrong with auth"),
            Err(err) => {
                assert!(err.is::<reqwest::Error>())
            }
        }
    }

    #[tokio::test]
    async fn test_submit_job_success() {
        let api_key: String = std::env::var("API_KEY").unwrap();
        let batch_client = BatchClient::new(&api_key, None).unwrap();

        let job_res = submit_job_util(&batch_client).await.unwrap();
        assert!(!job_res.id.is_empty())
    }

    #[tokio::test]
    async fn test_get_job() {
        let api_key: String = std::env::var("API_KEY").unwrap();
        let batch_client = BatchClient::new(&api_key, None).unwrap();

        let job_res = submit_job_util(&batch_client).await.unwrap();
        let get_job_res = batch_client.get_job(&job_res.id).await.unwrap();
        assert!(!get_job_res.job.data_name.is_empty());
        if let Some(dur) = get_job_res.job.duration {
            assert!(dur > 0);
        } else {
            unreachable!()
        }
    }

    #[tokio::test]
    async fn test_get_jobs() {
        let api_key: String = std::env::var("API_KEY").unwrap();
        let batch_client = BatchClient::new(&api_key, None).unwrap();

        let _ = submit_job_util(&batch_client).await.unwrap();
        let _ = submit_job_util(&batch_client).await.unwrap();
        let _ = submit_job_util(&batch_client).await.unwrap();
        let job_res = batch_client.get_jobs(Some(2), None).await.unwrap();
        assert!(job_res.jobs.len() == 2)
    }

    #[tokio::test]
    async fn test_get_json_result() {
        let api_key: String = std::env::var("API_KEY").unwrap();
        let batch_client = BatchClient::new(&api_key, None).unwrap();

        let job_res = submit_job_util(&batch_client).await.unwrap();
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
        let get_result_res = batch_client.get_json_result(&job_res.id).await.unwrap();
        assert!(get_result_res.job.data_name == "example.wav");
        assert!(get_result_res.results.len() != 0)
    }

    #[tokio::test]
    async fn test_get_text_result() {
        let api_key: String = std::env::var("API_KEY").unwrap();
        let batch_client = BatchClient::new(&api_key, None).unwrap();

        let job_res = submit_job_util(&batch_client).await.unwrap();
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
        let get_result_res = batch_client.get_text_result(&job_res.id).await.unwrap();
        assert!(get_result_res.len() != 0)
    }

    #[tokio::test]
    async fn test_delete_job() {
        let api_key: String = std::env::var("API_KEY").unwrap();
        let batch_client = BatchClient::new(&api_key, None).unwrap();

        let job_res = submit_job_util(&batch_client).await.unwrap();
        let delete_res = batch_client
            .delete_job(&job_res.id, Some(true))
            .await
            .unwrap();
        assert!(delete_res.job.status == models::job_details::Status::Deleted);
    }
}
