
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemStatusLastWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_sent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for ItemStatusLastWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}