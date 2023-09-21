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
pub struct RecognitionResult {
    #[serde(rename = "alternatives", skip_serializing_if = "Option::is_none")]
    pub alternatives: Option<Vec<crate::realtime::models::RecognitionAlternative>>,
    #[serde(rename = "attaches_to", skip_serializing_if = "Option::is_none")]
    pub attaches_to: Option<AttachesTo>,
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "end_time")]
    pub end_time: f32,
    #[serde(rename = "is_eos", skip_serializing_if = "Option::is_none")]
    pub is_eos: Option<bool>,
    #[serde(rename = "score", skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    #[serde(rename = "start_time")]
    pub start_time: f32,
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl RecognitionResult {
    pub fn new(end_time: f32, start_time: f32, r#type: RHashType) -> RecognitionResult {
        RecognitionResult {
            alternatives: None,
            attaches_to: None,
            channel: None,
            end_time,
            is_eos: None,
            score: None,
            start_time,
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AttachesTo {
    #[serde(rename = "next")]
    Next,
    #[serde(rename = "previous")]
    Previous,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "both")]
    Both,
}

impl Default for AttachesTo {
    fn default() -> AttachesTo {
        Self::Next
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "word")]
    Word,
    #[serde(rename = "punctuation")]
    Punctuation,
    #[serde(rename = "speaker_change")]
    SpeakerChange,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Word
    }
}
