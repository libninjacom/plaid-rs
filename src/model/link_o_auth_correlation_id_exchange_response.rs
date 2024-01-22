use serde::{Serialize, Deserialize};
///LinkOAuthCorrelationIdExchangeResponse defines the response schema for `/link/oauth/correlation_id/exchange`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkOAuthCorrelationIdExchangeResponse {
    ///The `link_token` associated to the given `link_correlation_id`, which can be used to re-initialize Link.
    pub link_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for LinkOAuthCorrelationIdExchangeResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}