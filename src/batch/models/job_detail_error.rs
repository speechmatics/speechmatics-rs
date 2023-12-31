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
pub struct JobDetailError {
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "message")]
    pub message: String,
}

impl JobDetailError {
    pub fn new(timestamp: String, message: String) -> JobDetailError {
        JobDetailError {
            timestamp,
            message,
        }
    }
}


