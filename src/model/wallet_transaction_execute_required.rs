
use serde::{Serialize, Deserialize};
use super::{WalletTransactionCounterparty, WalletTransactionAmount};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransactionExecuteRequired {
    pub amount: WalletTransactionAmount,
    pub counterparty: WalletTransactionCounterparty,
    pub idempotency_key: String,
    pub reference: String,
    pub wallet_id: String,
}
impl std::fmt::Display for WalletTransactionExecuteRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}