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
pub struct EndOfStream {
    #[serde(rename = "last_seq_no")]
    pub last_seq_no: i32,
    #[serde(rename = "message")]
    pub message: Message,
}

impl EndOfStream {
    pub fn new(last_seq_no: i32, message: Message) -> EndOfStream {
        EndOfStream {
            last_seq_no,
            message,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Message {
    #[serde(rename = "EndOfStream")]
    EndOfStream,
}

impl Default for Message {
    fn default() -> Message {
        Self::EndOfStream
    }
}

