use serde::{Serialize, Deserialize};
use super::BankTransferEvent;
///Defines the response schema for `/bank_transfer/event/sync`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferEventSyncResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bank_transfer_events: Vec<BankTransferEvent>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BankTransferEventSyncResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}