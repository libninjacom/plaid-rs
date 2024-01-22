use serde::{Serialize, Deserialize};
use super::{AccountIdentityMatchScore, Item};
///IdentityMatchResponse defines the response schema for `/identity/match`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityMatchResponse {
    ///The accounts for which Identity match has been requested
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<AccountIdentityMatchScore>,
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for IdentityMatchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}