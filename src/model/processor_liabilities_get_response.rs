use serde::{Serialize, Deserialize};
use super::{AccountBase, LiabilitiesObject};
///ProcessorLiabilitiesGetResponse defines the response schema for `/processor/liabilities/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorLiabilitiesGetResponse {
    ///A single account at a financial institution.
    pub account: AccountBase,
    ///An object containing liability accounts
    pub liabilities: LiabilitiesObject,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ProcessorLiabilitiesGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}