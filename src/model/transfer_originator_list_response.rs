use serde::{Serialize, Deserialize};
use super::Originator;
///Defines the response schema for `/transfer/originator/list`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferOriginatorListResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub originators: Vec<Originator>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferOriginatorListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}