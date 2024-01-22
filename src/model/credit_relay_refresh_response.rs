use serde::{Serialize, Deserialize};
///CreditRelayRefreshResponse defines the response schema for `/credit/relay/refresh`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditRelayRefreshResponse {
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset_report_id: Option<String>,
    pub relay_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditRelayRefreshResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}