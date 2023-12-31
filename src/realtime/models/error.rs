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
pub struct Error {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(rename = "message")]
    pub message: Message,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "seq_no", skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<i32>,
    #[serde(rename = "type")]
    pub type_value: Type,
}

impl Error {
    pub fn new(message: Message, reason: String, type_value: Type) -> Error {
        Error {
            code: None,
            message,
            reason,
            seq_no: None,
            type_value,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Message {
    #[serde(rename = "Error")]
    Error,
}

impl Default for Message {
    fn default() -> Message {
        Self::Error
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "invalid_message")]
    InvalidMessage,
    #[serde(rename = "invalid_model")]
    InvalidModel,
    #[serde(rename = "invalid_config")]
    InvalidConfig,
    #[serde(rename = "invalid_audio_type")]
    InvalidAudioType,
    #[serde(rename = "not_authorised")]
    NotAuthorised,
    #[serde(rename = "insufficient_funds")]
    InsufficientFunds,
    #[serde(rename = "not_allowed")]
    NotAllowed,
    #[serde(rename = "job_error")]
    JobError,
    #[serde(rename = "data_error")]
    DataError,
    #[serde(rename = "buffer_error")]
    BufferError,
    #[serde(rename = "protocol_error")]
    ProtocolError,
    #[serde(rename = "unknown_error")]
    UnknownError,
}

impl Default for Type {
    fn default() -> Type {
        Self::InvalidMessage
    }
}

