use serde::{Serialize, Deserialize};
use super::Transfer;
///Defines the response schema for `/transfer/list`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferListResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transfers: Vec<Transfer>,
}
impl std::fmt::Display for TransferListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}