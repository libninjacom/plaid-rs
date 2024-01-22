use serde::{Serialize, Deserialize};
use super::DetailedOriginator;
///Defines the response schema for `/transfer/originator/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferOriginatorGetResponse {
    ///Originator and their status.
    pub originator: DetailedOriginator,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferOriginatorGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}