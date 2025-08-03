use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LabRecord {
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

    #[serde(rename = "Value")]
    pub value: String,

    #[serde(rename = "Secondary Value")]
    pub secondary_value: String,

    #[serde(rename = "Expanded Value")]
    pub expanded_value: String,

    #[validate(length(min = 1))]
    #[serde(rename = "Report Date")]
    pub report_date: String,

    #[serde(rename = "LOINC")]
    pub loinc: String,

    #[serde(rename = "Normal Range of Value")]
    pub normal_range_of_value: String,

    #[serde(rename = "Result Evaluation/Flag")]
    pub result_evaluation_flag: String,

    #[serde(rename = "Unit of measurement")]
    pub unit_of_measurement: String,

    #[serde(rename = "Clinician Name")]
    pub clinician_name: String,

    #[serde(rename = "Specialty Code")]
    pub specialty_code: String,

    #[serde(rename = "Clinician Specialty")]
    pub clinician_specialty: String,

    #[serde(rename = "Hyperlink")]
    pub hyperlink: String,

    #[serde(rename = "HyperlinkUrl")]
    pub hyperlink_url: String,
}
