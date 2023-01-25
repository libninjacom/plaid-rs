
use serde::{Serialize, Deserialize};
use super::BankTransfer;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferListResponse {
    pub bank_transfers: Vec<BankTransfer>,
    pub request_id: String,
}
impl std::fmt::Display for BankTransferListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}