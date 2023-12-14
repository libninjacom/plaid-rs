
use serde::{Serialize, Deserialize};
use super::BaseReportAccount;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseReportItem {
    pub accounts: Vec<BaseReportAccount>,
    pub date_last_updated: chrono::DateTime<chrono::Utc>,
    pub institution_id: String,
    pub institution_name: String,
}
impl std::fmt::Display for BaseReportItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}