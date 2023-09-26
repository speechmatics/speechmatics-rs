/*
 * Speechmatics ASR REST API
 *
 * The Speechmatics Automatic Speech Recognition REST API is used to submit ASR jobs and receive the results. 
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@speechmatics.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateJobResponse {
    /// The unique ID assigned to the job. Keep a record of this for later retrieval of your completed job.
    #[serde(rename = "id")]
    pub id: String,
}

impl CreateJobResponse {
    pub fn new(id: String) -> CreateJobResponse {
        CreateJobResponse {
            id,
        }
    }
}

