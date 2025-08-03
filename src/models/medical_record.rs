use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::{InsuredInfo, LabRecord, LifestyleCondition, RxRecord, TestRecord};

#[derive(Debug, Deserialize, Serialize, Validate)]
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
    pub lifestyle_condition: Vec<LifestyleCondition>,
}
