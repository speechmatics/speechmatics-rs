use anyhow::Result;
use models::*;
use reqwest::{
    multipart::{Form, Part},
    Client,
    header::{AUTHORIZATION, HeaderMap, HeaderValue}
};
use std::fs;
use url::Url;

pub mod models;

pub const DEFAULT_BATCH_URL: &str = "https://asr.api.speechmatics.com/v2/";

pub struct BatchClient {
    batch_url: Url,
    client: Client,
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
        Ok(Self {
            batch_url: set_url,
            client,
        })
    }

    pub async fn submit_job(
        &self,
        config: JobConfig,
        file_name: std::path::PathBuf,
    ) -> Result<CreateJobResponse> {
        let url = self.batch_url.join("jobs")?;
        let file = fs::read(file_name).unwrap();
        let config_text = serde_json::to_string(&config)?;

        let file_part = Part::bytes(file);
        let form = Form::new()
            .part("data_file", file_part)
            .part("config", Part::text(config_text));

        let res = self
            .client
            .post(url)
            .multipart(form)
            .send()
            .await;
        let result = res?.bytes().await?;

        let serde_res = serde_json::from_slice::<CreateJobResponse>(&result)?;
        Ok(serde_res)
    }

    pub async fn get_job(&self, job_id: &str) -> Result<RetrieveJobResponse> {
        let url = self.batch_url.join("jobs")?.join(job_id)?;

        let res = self
            .client
            .get(url)
            .send()
            .await;
        let result = res?.bytes().await?;

        let serde_res = serde_json::from_slice::<RetrieveJobResponse>(&result)?;
        Ok(serde_res)
    }

    pub async fn get_jobs(
        &self,
        limit: Option<i32>,
        include_deleted: Option<bool>,
    ) -> Result<RetrieveJobsResponse> {
        let url = self.batch_url.join("jobs")?;

        let mut queries = vec![];

        if let Some(lim) = limit {
            queries.push(("limit", format!("{}", lim)))
        }

        if let Some(del) = include_deleted {
            queries.push(("include_deleted", format!("{}", del)))
        }

        let res = self
            .client
            .get(url)
            .query(&queries)
            .send()
            .await;
        let result = res?.bytes().await?;
        println!("{}", String::from_utf8(result.to_vec())?);

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
            .join("jobs")?
            .join(job_id)?
            .join("transcript")?;


        let mut queries = vec![];

        if let Some(form) = format {
            queries.push(("format", form))
        }

        let res = self
            .client
            .get(url)
            .query(&queries)
            .send()
            .await;
        let result = res?.bytes().await?;

        let serde_res = serde_json::from_slice::<RetrieveTranscriptResponse>(&result)?;
        Ok(serde_res)
    }

    pub async fn delete_job(&self, job_id: &str, force: Option<bool>) -> Result<DeleteJobResponse> {
        let url = self.batch_url.join("jobs")?.join(job_id)?;

        let mut queries = vec![];

        if let Some(force_set) = force {
            queries.push(("force", force_set))
        }

        let res = self
            .client
            .delete(url)
            .query(&queries)
            .send()
            .await;
        let result = res?.bytes().await?;

        println!("{}", String::from_utf8(result.to_vec())?);
        let serde_res = serde_json::from_slice::<DeleteJobResponse>(&result)?;
        Ok(serde_res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_submit_job_success() {
        let batch_client =
            BatchClient::new("R9rfOOCk5ssqGxlSuazkirAQkDqd7jcp", None).unwrap();

        let test_file_path = PathBuf::new()
            .join("..")
            .join("tests")
            .join("data")
            .join("example.wav");

        let config = JobConfig::default();
        let job_res = batch_client
            .submit_job(config, test_file_path)
            .await
            .unwrap();
        println!("{:?}", job_res);
    }

    #[tokio::test]
    async fn test_get_jobs() {
        let batch_client =
            BatchClient::new("R9rfOOCk5ssqGxlSuazkirAQkDqd7jcp", None).unwrap();

        let job_res = batch_client
            .get_jobs(Some(1), None)
            .await
            .unwrap();
        println!("{:?}", job_res);
    }
}
