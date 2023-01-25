
use serde::{Serialize, Deserialize};
use super::{AssetReportItem, AssetReportUser};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReport {
    pub asset_report_id: String,
    pub client_report_id: Option<String>,
    pub date_generated: String,
    pub days_requested: f64,
    pub items: Vec<AssetReportItem>,
    pub user: AssetReportUser,
}
impl std::fmt::Display for AssetReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}