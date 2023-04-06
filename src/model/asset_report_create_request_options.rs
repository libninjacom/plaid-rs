
use serde::{Serialize, Deserialize};
use super::AssetReportUser;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportCreateRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_report_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_fast_report: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<AssetReportUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}
impl std::fmt::Display for AssetReportCreateRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}