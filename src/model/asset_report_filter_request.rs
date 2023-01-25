
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportFilterRequest {
    pub account_ids_to_exclude: Vec<String>,
    pub asset_report_token: String,
}
impl std::fmt::Display for AssetReportFilterRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}