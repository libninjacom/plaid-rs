use serde::{Serialize, Deserialize};
use super::{Item, ItemStatus};
///ItemGetResponse defines the response schema for `/item/get` and `/item/webhook/update`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemGetResponse {
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///Information about the last successful and failed transactions update for the Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}
impl std::fmt::Display for ItemGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}