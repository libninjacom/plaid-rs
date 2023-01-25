
use serde::{Serialize, Deserialize};
use super::BankTransferEvent;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferEventSyncResponse {
    pub bank_transfer_events: Vec<BankTransferEvent>,
    pub request_id: String,
}
impl std::fmt::Display for BankTransferEventSyncResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}