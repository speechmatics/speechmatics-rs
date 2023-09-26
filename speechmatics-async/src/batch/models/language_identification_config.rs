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
pub struct LanguageIdentificationConfig {
    #[serde(rename = "expected_languages", skip_serializing_if = "Option::is_none")]
    pub expected_languages: Option<Vec<String>>,
}

impl LanguageIdentificationConfig {
    pub fn new() -> LanguageIdentificationConfig {
        LanguageIdentificationConfig {
            expected_languages: None,
        }
    }
}


