use serde::{Serialize, Deserialize};
use super::AccountBase;
///ProcessorBalanceGetResponse defines the response schema for `/processor/balance/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorBalanceGetResponse {
    ///A single account at a financial institution.
    pub account: AccountBase,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ProcessorBalanceGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}