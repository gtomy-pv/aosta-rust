use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationErrorType {
    FeatureNotFound,
    FeatureImpairmentPairInvalid,
    SelectOptionInvalid,
    NumericMissingValue,
    BloodPressureFormatInvalid,
}

impl fmt::Display for ValidationErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationErrorType::FeatureNotFound => write!(f, "FEATURE_NOT_FOUND"),
            ValidationErrorType::FeatureImpairmentPairInvalid => {
                write!(f, "FEATURE_IMPAIRMENT_PAIR_INVALID")
            }
            ValidationErrorType::SelectOptionInvalid => write!(f, "SELECT_OPTION_INVALID"),
            ValidationErrorType::NumericMissingValue => write!(f, "NUMERIC_MISSING_VALUE"),
            ValidationErrorType::BloodPressureFormatInvalid => {
                write!(f, "BLOOD_PRESSURE_FORMAT_INVALID")
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub error_type: ValidationErrorType,
    pub feature: String,
    pub impairment: String,
    pub value: Option<String>,
    pub secondary_value: Option<String>,
    pub message: String,
    pub tab: String,
}

impl ValidationError {
    pub fn new(
        error_type: ValidationErrorType,
        feature: String,
        impairment: String,
        value: Option<String>,
        secondary_value: Option<String>,
        message: String,
        tab: String,
    ) -> Self {
        Self {
            error_type,
            feature,
            impairment,
            value,
            secondary_value,
            message,
            tab,
        }
    }
}
