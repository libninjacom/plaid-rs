
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransaction {
    pub amount: WalletTransactionAmount,
    pub counterparty: WalletTransactionCounterparty,
    pub created_at: String,
    pub last_status_update: String,
    pub payment_id: Option<String>,
    pub reference: String,
    pub status: String,
    pub transaction_id: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for WalletTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}