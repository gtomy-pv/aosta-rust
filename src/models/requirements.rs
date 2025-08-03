use serde::{Deserialize, Serialize};
use thiserror::Error;
use tiberius::{Client, Config, Query};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[derive(Error, Debug)]
pub enum RequirementsError {
    #[error("Database connection failed: {0}")]
    DatabaseConnection(#[from] tiberius::error::Error),

    #[error("Network connection failed: {0}")]
    NetworkConnection(#[from] std::io::Error),

    #[error("Missing required field in database row")]
    MissingField,

    #[error("No requirements found in version {0}")]
    NoDataFound(i32),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequirementRow {
    pub tab: String,
    pub impairment: String,
    pub feature: String,
    pub value_type: String,
    pub select_option: Option<String>,
}

pub struct RequirementsService;

impl RequirementsService {
    pub async fn get_requirements(
        database_url: &str,
        version: i32,
    ) -> Result<Vec<RequirementRow>, RequirementsError> {
        let config = Config::from_ado_string(database_url)?;
        let tcp = TcpStream::connect(config.get_addr()).await?;
        let mut client = Client::connect(config, tcp.compat_write()).await?;

        let mut query = Query::new(
            r#"
            SELECT 
                LOWER(t.name) as tab,
                LOWER(i.name) as impairment,
                LOWER(f.name) as feature,
                LOWER(vt.name) as value_type,
                LOWER(so.name) as select_option
            FROM digitization_requirements.FeatureImpairment fi
            INNER JOIN digitization_requirements.Feature f ON fi.feature_id = f.id
            INNER JOIN digitization_requirements.Impairment i ON fi.impairment_id = i.id
            INNER JOIN digitization_requirements.FeatureType ft ON fi.feature_type_id = ft.id
            INNER JOIN digitization_requirements.ValueType vt ON fi.value_type_id = vt.id
            INNER JOIN digitization_requirements.Tab t ON fi.tab_id = t.id
            LEFT JOIN digitization_requirements.SelectOption so ON fi.id = so.feature_impairment_id
            INNER JOIN digitization_requirements.Version v ON fi.version_id = v.id
            WHERE v.number = @P1
        "#,
        );

        query.bind(version);
        let stream = query.query(&mut client).await?;
        let rows = stream.into_first_result().await?;

        if rows.is_empty() {
            return Err(RequirementsError::NoDataFound(version));
        }

        let mut requirements = Vec::new();

        for row in rows {
            let tab = row
                .get::<&str, &str>("tab")
                .ok_or(RequirementsError::MissingField)?
                .to_string();

            let impairment = row
                .get::<&str, &str>("impairment")
                .ok_or(RequirementsError::MissingField)?
                .to_string();

            let feature = row
                .get::<&str, &str>("feature")
                .ok_or(RequirementsError::MissingField)?
                .to_string();

            let value_type = row
                .get::<&str, &str>("value_type")
                .ok_or(RequirementsError::MissingField)?
                .to_string();

            let select_option = row
                .get::<&str, &str>("select_option")
                .map(|s| s.to_string());

            requirements.push(RequirementRow {
                tab,
                impairment,
                feature,
                value_type,
                select_option,
            });
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
                "Server={server};Database={database};User Id={username};Password={password};TrustServerCertificate=true;"
            ))
        } else {
            None
        }
    }

    #[tokio::test]
    async fn test_database_connection() {
        let database_url = match get_test_database_url() {
            Some(url) => url,
            None => {
                println!("Skipping test: Environment variables not set");
                return;
            }
        };

        let version = 24;
        let result = RequirementsService::get_requirements(&database_url, version).await;

        assert!(result.is_ok(), "Should successfully connect to database");
        println!("Database connection test passed");
    }

    #[tokio::test]
    async fn test_returns_data() {
        let database_url = match get_test_database_url() {
            Some(url) => url,
            None => {
                println!("Skipping test: Environment variables not set");
                return;
            }
        };

        let version = 24;
        let result = RequirementsService::get_requirements(&database_url, version).await;

        match result {
            Ok(requirements) => {
                assert!(!requirements.is_empty(), "Should return some data");

                let first_req = &requirements[0];
                assert!(!first_req.tab.is_empty(), "Tab should not be empty");
                assert!(!first_req.feature.is_empty(), "Feature should not be empty");

                println!("Retrieved {} requirements", requirements.len());
                println!(
                    "First requirement: tab={}, feature={}",
                    first_req.tab, first_req.feature
                );
            }
            Err(e) => panic!("Should return data but got error: {e:?}"),
        }
    }
}
