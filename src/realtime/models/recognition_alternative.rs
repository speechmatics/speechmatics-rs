/*
 * OpenAPI Template
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RecognitionAlternative {
    #[serde(rename = "confidence")]
    pub confidence: f32,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<Box<crate::realtime::models::RecognitionDisplay>>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "speaker", skip_serializing_if = "Option::is_none")]
    pub speaker: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl RecognitionAlternative {
    pub fn new(confidence: f32, content: String) -> RecognitionAlternative {
        RecognitionAlternative {
            confidence,
            content,
            display: None,
            language: None,
            speaker: None,
            tags: None,
        }
    }
}

