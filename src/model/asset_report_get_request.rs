
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportGetRequest {
    pub asset_report_token: String,
    pub fast_report: Option<bool>,
    pub include_insights: Option<bool>,
    pub options: Option<AssetReportGetRequestOptions>,
}
impl std::fmt::Display for AssetReportGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}