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
pub struct AlignmentConfig {
    #[serde(rename = "language")]
    pub language: String,
}

impl AlignmentConfig {
    pub fn new(language: String) -> AlignmentConfig {
        AlignmentConfig {
            language,
        }
    }
}

