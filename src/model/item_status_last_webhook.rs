
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemStatusLastWebhook {
    pub code_sent: Option<String>,
    pub sent_at: Option<String>,
}
impl std::fmt::Display for ItemStatusLastWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}