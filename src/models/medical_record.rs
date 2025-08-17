use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordType {
    #[serde(rename = "lab")]
    Lab,
    #[serde(rename = "lifestyle_condition")]
    LifestyleCondition,
    #[serde(rename = "test")]
    Test,
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct BaseRecord {
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

    #[serde(rename = "Hyperlink")]
    pub hyperlink: String,

    #[serde(rename = "HyperlinkUrl")]
    pub hyperlink_url: String,

    #[validate(length(min = 1))]
    #[serde(rename = "Clinical Code")]
    pub clinical_code: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ExtendedRecord {
    #[serde(flatten)]
    #[validate(nested)]
    pub base: BaseRecord,

    #[serde(rename = "Qualifier")]
    pub qualifier: String,

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
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LabRecord {
    #[serde(flatten)]
    #[validate(nested)]
    pub base: BaseRecord,

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
}

impl LabRecord {
    pub fn record_type(&self) -> RecordType {
        RecordType::Lab
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TestRecord {
    #[serde(flatten)]
    #[validate(nested)]
    pub extended: ExtendedRecord,

    #[serde(rename = "Location")]
    pub location: String,

    #[serde(rename = "Test Type")]
    pub test_type: String,
}

impl TestRecord {
    pub fn record_type(&self) -> RecordType {
        RecordType::Test
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LifestyleConditionRecord {
    #[serde(flatten)]
    #[validate(nested)]
    pub extended: ExtendedRecord,
}

impl LifestyleConditionRecord {
    pub fn record_type(&self) -> RecordType {
        RecordType::LifestyleCondition
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RxRecord {
    #[validate(length(min = 1))]
    #[serde(rename = "Impairment")]
    pub impairment: String,

    #[validate(length(min = 1))]
    #[serde(rename = "Rx Name")]
    pub rx_name: String,

    #[validate(length(min = 1))]
    #[serde(rename = "RX Type")]
    pub rx_type: String,

    #[serde(rename = "RxNORM")]
    pub rx_norm: String,

    #[validate(length(min = 1))]
    #[serde(rename = "Start Date")]
    pub start_date: String,

    #[serde(rename = "End Date")]
    pub end_date: Option<String>,

    #[validate(length(min = 1))]
    #[serde(rename = "Rx Date")]
    pub rx_date: String,

    #[serde(rename = "Hyperlink")]
    pub hyperlink: String,

    #[serde(rename = "HyperlinkUrl")]
    pub hyperlink_url: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
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

    #[serde(rename = "Street Address")]
    pub street_address: String,

    #[serde(rename = "State")]
    pub state: String,

    #[serde(rename = "Zip")]
    pub zip: String,

    #[serde(rename = "Most Recent Address Date")]
    pub most_recent_address_date: Option<String>,

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

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MedicalRecord {
    #[validate(nested)]
    #[serde(rename = "Insured_Info")]
    pub insured_info: InsuredInfo,

    #[validate(nested)]
    #[serde(rename = "Rx")]
    pub rx: Vec<RxRecord>,

    #[validate(nested)]
    #[serde(rename = "Lab")]
    pub lab: Vec<LabRecord>,

    #[validate(nested)]
    #[serde(rename = "Test")]
    pub test: Vec<TestRecord>,

    #[validate(nested)]
    #[serde(rename = "Lifestyle/Condition")]
    pub lifestyle_condition: Vec<LifestyleConditionRecord>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_portfolio_name"))]
pub struct ValidationRequest {
    #[validate(nested)]
    pub json_file: MedicalRecord,

    #[validate(length(min = 1, max = 10))]
    pub metadata_date: String,

    #[validate(length(min = 1))]
    pub metadata_insured_id: String,

    #[validate(length(min = 1))]
    pub metadata_insured_last_name: String,

    #[validate(length(min = 1))]
    pub metadata_insured_first_name: String,

    pub metadata_portfolio_name: String,

    #[validate(length(min = 1))]
    pub metadata_batch_name: String,

    #[validate(range(min = 1))]
    pub metadata_version: i32,
}

fn validate_portfolio_name(request: &ValidationRequest) -> Result<(), validator::ValidationError> {
    if request.metadata_batch_name == "Nightly" && request.metadata_portfolio_name.is_empty() {
        return Err(validator::ValidationError::new(
            "portfolio_name is required when batch_name is 'Nighlty",
        ));
    }
    Ok(())
}
