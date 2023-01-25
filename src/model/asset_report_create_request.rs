
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportCreateRequest {
    pub access_tokens: Vec<String>,
    pub days_requested: i64,
    pub options: Option<AssetReportCreateRequestOptions>,
    pub report_type: Option<String>,
}
impl std::fmt::Display for AssetReportCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}