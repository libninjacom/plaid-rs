
use serde::{Serialize, Deserialize};
use super::TransferEvent;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferEventListResponse {
    pub request_id: String,
    pub transfer_events: Vec<TransferEvent>,
}
impl std::fmt::Display for TransferEventListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}