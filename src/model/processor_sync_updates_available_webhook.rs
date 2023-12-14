
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorSyncUpdatesAvailableWebhook {
    pub account_id: String,
    pub environment: String,
    pub historical_update_complete: bool,
    pub initial_update_complete: bool,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for ProcessorSyncUpdatesAvailableWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}