use serde::{Serialize, Deserialize};
///DepositSwitchTokenCreateResponse defines the response schema for `/deposit_switch/token/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositSwitchTokenCreateResponse {
    ///Deposit switch token, used to initialize Link for the Deposit Switch product
    pub deposit_switch_token: String,
    ///Expiration time of the token, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format
    pub deposit_switch_token_expiration_time: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for DepositSwitchTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}