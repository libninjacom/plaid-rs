use serde::{Serialize, Deserialize};
use super::TransferIntentGet;
///Defines the response schema for `/transfer/intent/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferIntentGetResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///Represents a transfer intent within Transfer UI.
    pub transfer_intent: TransferIntentGet,
}
impl std::fmt::Display for TransferIntentGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}