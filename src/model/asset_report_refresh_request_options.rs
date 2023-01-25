
use serde::{Serialize, Deserialize};
use super::AssetReportUser;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportRefreshRequestOptions {
    pub client_report_id: Option<String>,
    pub user: Option<AssetReportUser>,
    pub webhook: Option<String>,
}
impl std::fmt::Display for AssetReportRefreshRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}