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
pub struct StartRecognition {
    #[serde(rename = "audio_format")]
    pub audio_format: Box<crate::realtime::models::AudioFormat>,
    #[serde(rename = "message")]
    pub message: Message,
    #[serde(rename = "transcription_config")]
    pub transcription_config: Box<crate::realtime::models::TranscriptionConfig>,
    #[serde(rename = "translation_config", skip_serializing_if = "Option::is_none")]
    pub translation_config: Option<Box<crate::realtime::models::TranslationConfig>>,
}

impl StartRecognition {
    pub fn new(audio_format: crate::realtime::models::AudioFormat, message: Message, transcription_config: crate::realtime::models::TranscriptionConfig) -> StartRecognition {
        StartRecognition {
            audio_format: Box::new(audio_format),
            message,
            transcription_config: Box::new(transcription_config),
            translation_config: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Message {
    #[serde(rename = "StartRecognition")]
    StartRecognition,
}

impl Default for Message {
    fn default() -> Message {
        Self::StartRecognition
    }
}

