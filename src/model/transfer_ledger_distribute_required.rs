
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferLedgerDistributeRequired {
    pub amount: String,
    pub from_client_id: String,
    pub idempotency_key: String,
    pub to_client_id: String,
}
impl std::fmt::Display for TransferLedgerDistributeRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}