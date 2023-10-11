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
pub struct SetRecognitionConfig {
    #[serde(rename = "message")]
    pub message: Message,
    #[serde(rename = "transcription_config")]
    pub transcription_config: Box<crate::realtime::models::TranscriptionConfig>,
}

impl SetRecognitionConfig {
    pub fn new(message: Message, transcription_config: crate::realtime::models::TranscriptionConfig) -> SetRecognitionConfig {
        SetRecognitionConfig {
            message,
            transcription_config: Box::new(transcription_config),
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Message {
    #[serde(rename = "SetRecognitionConfig")]
    SetRecognitionConfig,
}

impl Default for Message {
    fn default() -> Message {
        Self::SetRecognitionConfig
    }
}
