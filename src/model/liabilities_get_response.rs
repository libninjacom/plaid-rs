use serde::{Serialize, Deserialize};
use super::{AccountBase, Item, LiabilitiesObject};
///LiabilitiesGetResponse defines the response schema for `/liabilities/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LiabilitiesGetResponse {
    ///An array of accounts associated with the Item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<AccountBase>,
    ///Metadata about the Item.
    pub item: Item,
    ///An object containing liability accounts
    pub liabilities: LiabilitiesObject,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for LiabilitiesGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}