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
pub struct AudioFormat {
    #[serde(rename = "encoding", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encoding>,
    #[serde(rename = "sample_rate", skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i32>,
    #[serde(rename = "type")]
    pub type_value: Type,
}

impl AudioFormat {
    pub fn new(type_value: Type) -> AudioFormat {
        AudioFormat {
            encoding: None,
            sample_rate: None,
            type_value,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Encoding {
    #[serde(rename = "pcm_f32le")]
    PcmF32le,
    #[serde(rename = "pcm_s16le")]
    PcmS16le,
    #[serde(rename = "mulaw")]
    Mulaw,
}

impl Default for Encoding {
    fn default() -> Encoding {
        Self::PcmF32le
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "file")]
    File,
}

impl Default for Type {
    fn default() -> Type {
        Self::Raw
    }
}

