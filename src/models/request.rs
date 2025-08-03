use crate::models::MedicalRecord;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Serialize, Validate)]
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

fn validate_portfolio_name(request: &ValidationRequest) -> Result<(), ValidationError> {
    if request.metadata_batch_name == "Nightly" && request.metadata_portfolio_name.is_empty() {
        return Err(ValidationError::new(
            "portfolio_name is required when batch_name is 'Nighlty' ",
        ));
    }

    Ok(())
}
