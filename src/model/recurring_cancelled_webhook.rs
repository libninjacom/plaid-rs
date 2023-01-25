
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecurringCancelledWebhook {
    pub environment: String,
    pub recurring_transfer_id: String,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for RecurringCancelledWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}