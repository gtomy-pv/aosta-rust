use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct InsuredInfo {
    #[validate(length(min = 1))]
    #[serde(rename = "Insured ID")]
    pub insured_id: String,

    #[validate(length(min = 1))]
    #[serde(rename = "Policy ID")]
    pub policy_id: String,

    #[validate(length(min = 1))]
    #[serde(rename = "First Name")]
    pub first_name: String,

    #[serde(rename = "Middle Name")]
    pub middle_name: String,

    #[validate(length(min = 1))]
    #[serde(rename = "Last Name")]
    pub last_name: String,

    #[validate(length(min = 1))]
    #[serde(rename = "Gender")]
    pub gender: String,

    #[validate(length(min = 1))]
    #[serde(rename = "Date of Birth")]
    pub date_of_birth: String,

    #[serde(rename = "Ethnicity")]
    pub ethnicity: String,

    #[serde(rename = "Date of Death")]
    pub date_of_death: Option<String>,

    #[validate(length(min = 1))]
    #[serde(rename = "Insured SSN")]
    pub insured_ssn: String,

    #[validate(length(min = 1))]
    #[serde(rename = "Street Address")]
    pub street_address: String,

    #[validate(length(min = 1))]
    #[serde(rename = "State")]
    pub state: String,

    #[validate(length(min = 1))]
    #[serde(rename = "Zip")]
    pub zip: String,

    #[serde(rename = "Most Recent Address Date")]
    pub most_recent_address_date: String,

    #[serde(rename = "Medical Records Start Date")]
    pub medical_records_start_date: String,

    #[serde(rename = "Medical Records End Date")]
    pub medical_records_end_date: String,

    #[serde(rename = "Medical Record Files")]
    pub medical_record_files: String,

    #[serde(rename = "File Status")]
    pub file_status: HashMap<String, String>,

    #[serde(rename = "AddressHyperlink")]
    pub address_hyperlink: String,

    #[serde(rename = "AddressHyperlinkUrl")]
    pub address_hyperlink_url: String,

    #[serde(rename = "Requirement version")]
    pub requirement_version: String,

    #[serde(rename = "Primary Extractor")]
    pub primary_extractor: String,

    #[serde(rename = "Primary Auditor")]
    pub primary_auditor: String,

    #[serde(rename = "Secondary Auditor")]
    pub secondary_auditor: String,

    #[serde(rename = "Batch")]
    pub batch: String,

    #[serde(rename = "Set")]
    pub set: String,

    #[serde(rename = "Aosta Team")]
    pub aosta_team: String,
}
