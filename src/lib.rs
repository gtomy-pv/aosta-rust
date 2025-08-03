pub mod models;

#[cfg(test)]
mod tests {
    use std::fs;

    use validator::Validate;

    use crate::models::ValidationRequest;

    #[test]
    fn test_sample_json_parses() {
        let json_data = fs::read_to_string("sample_data/test_aosta_payload.json")
            .expect("Could not read the file");

        let request: ValidationRequest =
            serde_json::from_str(&json_data).expect("JSON should parse successfully.");

        if let Err(errors) = request.validate() {
            panic!("Validation should pass: {errors:#?}");
        }

        println!("JSON parsed and validated successfully");
    }
}
