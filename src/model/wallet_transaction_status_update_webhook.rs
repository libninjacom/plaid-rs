
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletTransactionStatusUpdateWebhook {
    pub environment: String,
    pub new_status: String,
    pub old_status: String,
    pub timestamp: String,
    pub transaction_id: String,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for WalletTransactionStatusUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}