
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransactionStatusUpdateWebhook {
    pub environment: String,
    pub new_status: String,
    pub old_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for WalletTransactionStatusUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}