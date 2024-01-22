use serde::{Serialize, Deserialize};
use super::Transfer;
///Defines the response schema for `/transfer/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferGetResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///Represents a transfer within the Transfers API.
    pub transfer: Transfer,
}
impl std::fmt::Display for TransferGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}