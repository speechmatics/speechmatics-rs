/*
 * OpenAPI Template
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServerMessages {
    #[serde(rename = "RecognitionStarted")]
    RecognitionStarted,
    #[serde(rename = "AudioAdded")]
    AudioAdded,
    #[serde(rename = "AddPartialTranscript")]
    AddPartialTranscript,
    #[serde(rename = "AddTranscript")]
    AddTranscript,
    #[serde(rename = "AddPartialTranslation")]
    AddPartialTranslation,
    #[serde(rename = "AddTranslation")]
    AddTranslation,
    #[serde(rename = "EndOfTranscript")]
    EndOfTranscript,
    #[serde(rename = "Info")]
    Info,
    #[serde(rename = "Warning")]
    Warning,
    #[serde(rename = "Error")]
    Error,

}

impl ToString for ServerMessages {
    fn to_string(&self) -> String {
        match self {
            Self::RecognitionStarted => String::from("RecognitionStarted"),
            Self::AudioAdded => String::from("AudioAdded"),
            Self::AddPartialTranscript => String::from("AddPartialTranscript"),
            Self::AddTranscript => String::from("AddTranscript"),
            Self::AddPartialTranslation => String::from("AddPartialTranslation"),
            Self::AddTranslation => String::from("AddTranslation"),
            Self::EndOfTranscript => String::from("EndOfTranscript"),
            Self::Info => String::from("Info"),
            Self::Warning => String::from("Warning"),
            Self::Error => String::from("Error"),
        }
    }
}

impl Default for ServerMessages {
    fn default() -> ServerMessages {
        Self::RecognitionStarted
    }
}




