use serde::{Serialize, Deserialize};
///CreditRelayRemoveResponse defines the response schema for `/credit/relay/remove`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditRelayRemoveResponse {
    ///`true` if the relay token was successfully removed.
    pub removed: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditRelayRemoveResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}