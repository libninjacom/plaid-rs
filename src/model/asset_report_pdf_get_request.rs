
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportPdfGetRequest {
    pub asset_report_token: String,
    pub options: Option<AssetReportPdfGetRequestOptions>,
}
impl std::fmt::Display for AssetReportPdfGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}