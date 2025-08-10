use crate::models::RequirementRow;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct RequirementCache {
    pub version: i32,

    pub valid_features: HashSet<String>,
    pub valid_feature_impairment_paurs: HashSet<(String, String)>,
    pub feature_select_options: HashMap<String, HashSet<String>>,
    pub requirement_rows: Vec<RequirementRow>,
}
