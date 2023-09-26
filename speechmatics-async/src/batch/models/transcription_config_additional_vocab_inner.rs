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
pub struct TranscriptionConfigAdditionalVocabInner {
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "sounds_like", skip_serializing_if = "Option::is_none")]
    pub sounds_like: Option<Vec<String>>,
}

impl TranscriptionConfigAdditionalVocabInner {
    pub fn new(content: String) -> TranscriptionConfigAdditionalVocabInner {
        TranscriptionConfigAdditionalVocabInner {
            content,
            sounds_like: None,
        }
    }
}

