use serde::{Serialize, Deserialize};
use super::BaseReportItem;
///An object representing a Base Report
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseReport {
    ///The date and time when the Base Report was created, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (e.g. "2018-04-12T03:32:11Z").
    pub date_generated: chrono::DateTime<chrono::Utc>,
    ///The number of days of transaction history requested.
    pub days_requested: f64,
    ///Data returned by Plaid about each of the Items included in the Base Report.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<BaseReportItem>,
    ///A unique ID identifying an Base Report. Like all Plaid identifiers, this ID is case sensitive.
    pub report_id: String,
}
impl std::fmt::Display for BaseReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}