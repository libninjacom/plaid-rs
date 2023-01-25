
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecurringNewTransferWebhook {
    pub environment: String,
    pub recurring_transfer_id: String,
    pub transfer_id: String,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for RecurringNewTransferWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}