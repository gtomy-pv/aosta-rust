use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
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
