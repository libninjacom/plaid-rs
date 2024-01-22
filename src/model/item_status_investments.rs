use serde::{Serialize, Deserialize};
///Information about the last successful and failed investments update for the Item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemStatusInvestments {
    ///[ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of the last failed investments update for the Item. The status will update each time Plaid fails an attempt to connect with the institution, regardless of whether any new data is available in the update.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_failed_update: Option<chrono::DateTime<chrono::Utc>>,
    ///[ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of the last successful investments update for the Item. The status will update each time Plaid successfully connects with the institution, regardless of whether any new data is available in the update.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_successful_update: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for ItemStatusInvestments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}