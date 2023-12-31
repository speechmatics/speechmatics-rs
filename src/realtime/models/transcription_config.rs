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
pub struct TranscriptionConfig {
    #[serde(rename = "additional_vocab", skip_serializing_if = "Option::is_none")]
    pub additional_vocab: Option<Vec<crate::realtime::models::VocabWord>>,
    #[serde(rename = "diarization", skip_serializing_if = "Option::is_none")]
    pub diarization: Option<crate::realtime::models::DiarizationConfig>,
    /// Request a specialized model based on 'language' but optimized for a particular field, e.g. \"finance\" or \"medical\".
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "enable_entities", skip_serializing_if = "Option::is_none")]
    pub enable_entities: Option<bool>,
    #[serde(rename = "enable_partials", skip_serializing_if = "Option::is_none")]
    pub enable_partials: Option<bool>,
    #[serde(rename = "language")]
    pub language: String,
    #[serde(rename = "max_delay", skip_serializing_if = "Option::is_none")]
    pub max_delay: Option<f32>,
    #[serde(rename = "max_delay_mode", skip_serializing_if = "Option::is_none")]
    pub max_delay_mode: Option<crate::realtime::models::MaxDelayModeConfig>,
    #[serde(rename = "operating_point", skip_serializing_if = "Option::is_none")]
    pub operating_point: Option<crate::realtime::models::OperatingPoint>,
    #[serde(rename = "output_locale", skip_serializing_if = "Option::is_none")]
    pub output_locale: Option<String>,
    #[serde(rename = "punctuation_overrides", skip_serializing_if = "Option::is_none")]
    pub punctuation_overrides: Option<Box<crate::realtime::models::PunctuationOverrides>>,
    #[serde(rename = "speaker_change_sensitivity", skip_serializing_if = "Option::is_none")]
    pub speaker_change_sensitivity: Option<f32>,
    #[serde(rename = "speaker_diarization_config", skip_serializing_if = "Option::is_none")]
    pub speaker_diarization_config: Option<Box<crate::realtime::models::SpeakerDiarizationConfig>>,
}

impl TranscriptionConfig {
    pub fn new(language: String) -> TranscriptionConfig {
        TranscriptionConfig {
            additional_vocab: None,
            diarization: None,
            domain: None,
            enable_entities: None,
            enable_partials: None,
            language,
            max_delay: None,
            max_delay_mode: None,
            operating_point: None,
            output_locale: None,
            punctuation_overrides: None,
            speaker_change_sensitivity: None,
            speaker_diarization_config: None,
        }
    }
}


