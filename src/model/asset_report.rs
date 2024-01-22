use serde::{Serialize, Deserialize};
use super::{AssetReportItem, AssetReportUser};
///An object representing an Asset Report
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReport {
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    pub asset_report_id: String,
    ///An identifier you determine and submit for the Asset Report.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_report_id: Option<String>,
    ///The date and time when the Asset Report was created, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (e.g. "2018-04-12T03:32:11Z").
    pub date_generated: chrono::DateTime<chrono::Utc>,
    ///The duration of transaction history you requested
    pub days_requested: f64,
    ///Data returned by Plaid about each of the Items included in the Asset Report.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AssetReportItem>,
    ///The user object allows you to provide additional information about the user to be appended to the Asset Report. All fields are optional. The `first_name`, `last_name`, and `ssn` fields are required if you would like the Report to be eligible for Fannie Mae’s Day 1 Certainty™ program.
    pub user: AssetReportUser,
}
impl std::fmt::Display for AssetReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}