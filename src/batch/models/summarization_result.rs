/*
 * Speechmatics ASR REST API
 *
 * The Speechmatics Automatic Speech Recognition REST API is used to submit ASR jobs and receive the results. 
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@speechmatics.com
 * Generated by: https://openapi-generator.tech
 */

/// SummarizationResult : Summary of the transcript, configured using `summarization_config`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SummarizationResult {
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl SummarizationResult {
    /// Summary of the transcript, configured using `summarization_config`.
    pub fn new() -> SummarizationResult {
        SummarizationResult {
            content: None,
        }
    }
}


