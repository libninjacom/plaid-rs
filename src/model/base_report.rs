
use serde::{Serialize, Deserialize};
use super::BaseReportItem;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseReport {
    pub date_generated: chrono::DateTime<chrono::Utc>,
    pub days_requested: f64,
    pub items: Vec<BaseReportItem>,
    pub report_id: String,
}
impl std::fmt::Display for BaseReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}