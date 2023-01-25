
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportAuditCopyCreateRequest {
    pub asset_report_token: String,
    pub auditor_id: Option<String>,
}
impl std::fmt::Display for AssetReportAuditCopyCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}