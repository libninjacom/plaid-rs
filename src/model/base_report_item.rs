use serde::{Serialize, Deserialize};
use super::BaseReportAccount;
///A representation of an Item within a Base Report.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseReportItem {
    ///Data about each of the accounts open on the Item.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<BaseReportAccount>,
    ///The date and time when this Item’s data was last retrieved from the financial institution, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub date_last_updated: chrono::DateTime<chrono::Utc>,
    ///The id of the financial institution associated with the Item.
    pub institution_id: String,
    ///The full financial institution name associated with the Item.
    pub institution_name: String,
}
impl std::fmt::Display for BaseReportItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}