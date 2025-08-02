use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct TestRecord {
    #[validate(length(min = 1))]
    #[serde(rename = "Tab")]
    pub tab: String,

    #[validate(length(min = 1))]
    #[serde(rename = "Impairment")]
    pub impairment: String,

    #[validate(length(min = 1))]
    #[serde(rename = "Feature Type")]
    pub feature_type: String,

    #[validate(length(min = 1))]
    #[serde(rename = "Feature")]
    pub feature: String,

    #[validate(length(min = 1))]
    #[serde(rename = "Value Type")]
    pub value_type: String,

    #[validate(length(min = 1))]
    #[serde(rename = "Value")]
    pub value: String,

    #[serde(rename = "Secondary Value")]
    pub secondary_value: String,

    #[serde(rename = "Expanded Value")]
    pub expanded_value: String,

    #[serde(rename = "Qualifier")]
    pub qualifier: String,

    #[serde(rename = "Location")]
    pub location: String,

    #[serde(rename = "Test Type")]
    pub test_type: String,

    #[validate(length(min = 1))]
    #[serde(rename = "Report Date")]
    pub report_date: String,

    #[serde(rename = "Event Date")]
    pub event_date: Option<String>,

    #[serde(rename = "Imputed Event Date")]
    pub imputed_event_date: Option<String>,

    #[serde(rename = "Onset Date")]
    pub onset_date: Option<String>,

    #[serde(rename = "ICD Code")]
    pub icd_code: String,

    #[serde(rename = "Snomed Code")]
    pub snomed_code: String,

    #[serde(rename = "Condition Status")]
    pub condition_status: String,

    #[serde(rename = "Clinician Name")]
    pub clinician_name: String,

    #[serde(rename = "Specialty Code")]
    pub specialty_code: String,

    #[serde(rename = "Clinician Specialty")]
    pub clinician_specialty: String,

    #[validate(length(min = 1))]
    #[serde(rename = "Hyperlink")]
    pub hyperlink: String,

    #[validate(length(min = 1))]
    #[serde(rename = "HyperlinkUrl")]
    pub hyperlink_url: String,
}
