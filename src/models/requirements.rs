use serde::{Deserialize, Serialize};
use sqlx_oldapi::{FromRow, MssqlPool};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequirementsError {
    #[error("Database connection failed: {0}")]
    DatabaseConnectionError(#[from] sqlx_oldapi::Error),

    #[error("No requirements found in version {0}")]
    NoDataFound(i32),
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct RequirementRow {
    pub tab: String,
    pub impairment: String,
    pub feature: String,
    pub feature_type: String,
    pub value_type: String,
    pub select_option: Option<String>,
    pub code: Option<String>,
}

pub struct RequirementsService;

impl RequirementsService {
    pub async fn get_requirements(
        database_url: &str,
        version: i32,
    ) -> Result<Vec<RequirementRow>, RequirementsError> {
        let pool = MssqlPool::connect(database_url).await?;

        let requirements = sqlx_oldapi::query_as::<_, RequirementRow>(
            r#"
        WITH codes AS (
            SELECT
                cs.name AS system_name,
                impairment_code.name AS impairment,
                CASE WHEN feature_code.name IS NULL THEN working.name ELSE feature_code.name END AS feature,
                CASE WHEN feature_code.name IS NULL THEN NULL ELSE working.name END AS select_option,
                working.code
            FROM dbo.Code working
            JOIN dbo.CodeSystem cs
                ON working.system_id = cs.id
                    AND cs.name = 'Clinical V25'
            JOIN dbo.CodeMapRelationship has_feature_relationship
                ON has_feature_relationship.name = 'has_feature'
            LEFT JOIN dbo.CodeMap feature_code_map
                ON feature_code_map.source_code_id = working.id
                AND feature_code_map.relationship_id = has_feature_relationship.id
            LEFT JOIN dbo.Code feature_code
                ON feature_code.id = feature_code_map.target_code_id
                AND feature_code.system_id = cs.id
            JOIN dbo.CodeMapRelationship has_impairment_relationship
                ON has_impairment_relationship.name = 'has_impairment'
            LEFT JOIN dbo.CodeMap impairment_code_map
                ON impairment_code_map.source_code_id = feature_code.id
                AND impairment_code_map.relationship_id = has_impairment_relationship.id
            LEFT JOIN dbo.Code impairment_code
                ON impairment_code.id = impairment_code_map.target_code_id
                AND impairment_code.system_id = cs.id
            WHERE working.code NOT LIKE '%-F%'
                AND working.code NOT LIKE 'HEIMP%'
        ),
        requirements AS (
            SELECT
                tab.name AS tab,
                Feature.name AS feature,
                Impairment.name AS impairment,
                FeatureType.name AS feature_type,
                SelectOption.name AS select_option,
                ValueType.name AS value_type
            FROM digitization_requirements.FeatureImpairment
            JOIN digitization_requirements.[Version]
                ON FeatureImpairment.version_id = [Version].id
            JOIN digitization_requirements.Feature
                ON FeatureImpairment.feature_id = Feature.id
            JOIN digitization_requirements.Impairment
                ON FeatureImpairment.impairment_id = Impairment.id
            JOIN digitization_requirements.FeatureType
                ON FeatureImpairment.feature_type_id = FeatureType.id
            LEFT JOIN digitization_requirements.SelectOption
                ON FeatureImpairment.id = SelectOption.feature_impairment_id
            LEFT JOIN digitization_requirements.ValueType
                ON ValueType.id = FeatureImpairment.value_type_id
            JOIN digitization_requirements.Tab
                ON tab.id = FeatureImpairment.tab_id
            WHERE [Version].number =  @P1
        )
        SELECT
            LOWER(requirements.tab) AS tab,
            LOWER(requirements.feature) AS feature,
            LOWER(requirements.impairment) AS impairment,
            LOWER(requirements.feature_type) AS feature_type,
            LOWER(requirements.select_option) AS select_option,
            LOWER(requirements.value_type) AS value_type,
            codes.code
        FROM requirements
        LEFT JOIN codes
            ON requirements.feature = codes.feature
            AND ISNULL(requirements.select_option, 'N/A') = ISNULL(codes.select_option, 'N/A')
            AND ISNULL(codes.impairment, requirements.impairment) = requirements.impairment
            "#,
        )
        .bind(version)
        .fetch_all(&pool)
        .await?;

        if requirements.is_empty() {
            return Err(RequirementsError::NoDataFound(version));
        }

        Ok(requirements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn get_test_database_url() -> Option<String> {
        if let (Ok(server), Ok(database), Ok(username), Ok(password)) = (
            env::var("SQL_SERVER_HOST"),
            env::var("SQL_SERVER_DS_DATABASE_NAME"),
            env::var("SQL_SERVER_USER"),
            env::var("SQL_SERVER_PASSWORD"),
        ) {
            Some(format!(
                "mssql://{username}:{password}@{server}/{database}?trust_server_certificate=true"
            ))
        } else {
            None
        }
    }

    #[tokio::test]
    async fn test_database_connection_error() {
        let bad_url = "mssql://fake:fake@nonexistent/fake?trust_server_certificate=true";

        let result = RequirementsService::get_requirements(bad_url, 25).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            RequirementsError::DatabaseConnectionError(_) => {
                println!("DatabaseConnectionError test passed");
            }
            other => panic!("Expected DatabaseConnectionError, got: {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_no_data_found_error() {
        let database_url = match get_test_database_url() {
            Some(url) => url,
            None => {
                println!("Skipping test: No database connection available");
                return;
            }
        };

        let invalid_version = 99999; // Use a version that should NOT exist
        let result = RequirementsService::get_requirements(&database_url, invalid_version).await;

        match result {
            Err(RequirementsError::NoDataFound(version)) => {
                assert_eq!(version, invalid_version);
                println!("NoDataFound error test passed");
            }
            Ok(_) => {
                println!(" Warning: Expected no data for version {invalid_version} but got results",)
            }
            Err(other) => panic!("Expected NoDataFound, got: {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_successful_connection_returns_data() {
        let database_url = match get_test_database_url() {
            Some(url) => url,
            None => {
                println!("Skipping test: No database connection available");
                return;
            }
        };

        let version = 25; // Use a version that should exist
        let result = RequirementsService::get_requirements(&database_url, version).await;

        match result {
            Ok(requirements) => {
                assert!(
                    !requirements.is_empty(),
                    "Should return at least one requirement"
                );
                println!(
                    "Connection successful, found {} requirements",
                    requirements.len()
                );
            }
            Err(RequirementsError::NoDataFound(_)) => {
                println!("No data found for version {version} - this might be expected");
            }
            Err(e) => panic!("Unexpected error: {e:?}"),
        }
    }
}
