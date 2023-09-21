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
pub struct PunctuationOverrides {
    /// The punctuation marks which the client is prepared to accept in transcription output, or the special value 'all' (the default). Unsupported marks are ignored. This value is used to guide the transcription process.
    #[serde(rename = "permitted_marks", skip_serializing_if = "Option::is_none")]
    pub permitted_marks: Option<Vec<String>>,
    /// Ranges between zero and one. Higher values will produce more punctuation. The default is 0.5.
    #[serde(rename = "sensitivity", skip_serializing_if = "Option::is_none")]
    pub sensitivity: Option<f32>,
}

impl PunctuationOverrides {
    pub fn new() -> PunctuationOverrides {
        PunctuationOverrides {
            permitted_marks: None,
            sensitivity: None,
        }
    }
}


