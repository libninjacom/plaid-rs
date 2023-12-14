
use serde::{Serialize, Deserialize};
use super::TransferLedgerBalance;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferLedgerGetResponse {
    pub balance: TransferLedgerBalance,
    pub request_id: String,
}
impl std::fmt::Display for TransferLedgerGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}