use serde::{Serialize, Deserialize};
use super::{AccountIdentity, Item};
///IdentityGetResponse defines the response schema for `/identity/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityGetResponse {
    ///The accounts for which Identity data has been requested
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<AccountIdentity>,
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for IdentityGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}