/*
 * Speechmatics ASR REST API
 *
 * The Speechmatics Automatic Speech Recognition REST API is used to submit ASR jobs and receive the results. 
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@speechmatics.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UsageDetails {
    #[serde(rename = "mode")]
    pub mode: crate::batch::models::JobMode,
    #[serde(rename = "type")]
    pub type_value: crate::batch::models::JobType,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "operating_point", skip_serializing_if = "Option::is_none")]
    pub operating_point: Option<crate::batch::models::OperatingPoint>,
    /// Total number of billable jobs in this cycle
    #[serde(rename = "count")]
    pub count: i32,
    /// Total duration of billable jobs (in hours) this cycle
    #[serde(rename = "duration_hrs")]
    pub duration_hrs: f32,
}

impl UsageDetails {
    pub fn new(mode: crate::batch::models::JobMode, type_value: crate::batch::models::JobType, count: i32, duration_hrs: f32) -> UsageDetails {
        UsageDetails {
            mode,
            type_value,
            language: None,
            operating_point: None,
            count,
            duration_hrs,
        }
    }
}


