use serde::{Serialize, Deserialize};
///Information about the last webhook fired for the Item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemStatusLastWebhook {
    ///The last webhook code sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code_sent: Option<String>,
    ///[ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of when the webhook was fired.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for ItemStatusLastWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}