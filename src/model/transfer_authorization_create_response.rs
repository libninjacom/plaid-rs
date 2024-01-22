use serde::{Serialize, Deserialize};
use super::TransferAuthorization;
///Defines the response schema for `/transfer/authorization/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferAuthorizationCreateResponse {
    ///Contains the authorization decision for a proposed transfer.
    pub authorization: TransferAuthorization,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferAuthorizationCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}