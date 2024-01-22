use serde::{Serialize, Deserialize};
///CreditRelayCreateResponse defines the response schema for `/credit/relay/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditRelayCreateResponse {
    ///A token that can be shared with a third party to allow them to access the Asset Report. This token should be stored securely.
    pub relay_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditRelayCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}