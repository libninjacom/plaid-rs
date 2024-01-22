use serde::{Serialize, Deserialize};
use super::AccountIdentity;
///ProcessorIdentityGetResponse defines the response schema for `/processor/identity/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorIdentityGetResponse {
    ///Identity information about an account
    pub account: AccountIdentity,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ProcessorIdentityGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}