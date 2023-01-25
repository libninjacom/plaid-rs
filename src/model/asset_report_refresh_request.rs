
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportRefreshRequest {
    pub asset_report_token: String,
    pub days_requested: Option<i64>,
    pub options: Option<AssetReportRefreshRequestOptions>,
}
impl std::fmt::Display for AssetReportRefreshRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}