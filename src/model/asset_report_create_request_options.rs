
use serde::{Serialize, Deserialize};
use super::AssetReportUser;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportCreateRequestOptions {
    pub client_report_id: Option<String>,
    pub include_fast_report: Option<bool>,
    pub products: Option<Vec<String>>,
    pub user: Option<AssetReportUser>,
    pub webhook: Option<String>,
}
impl std::fmt::Display for AssetReportCreateRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}