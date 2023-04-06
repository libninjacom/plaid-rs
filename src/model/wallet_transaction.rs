
use serde::{Serialize, Deserialize};
use super::{WalletTransactionAmount, WalletTransactionCounterparty};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransaction {
    pub amount: WalletTransactionAmount,
    pub counterparty: WalletTransactionCounterparty,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_status_update: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    pub reference: String,
    pub status: String,
    pub transaction_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub wallet_id: String,
}
impl std::fmt::Display for WalletTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}