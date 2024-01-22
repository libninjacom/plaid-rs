use serde::{Serialize, Deserialize};
use super::Item;
///ItemWebhookUpdateResponse defines the response schema for `/item/webhook/update`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemWebhookUpdateResponse {
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ItemWebhookUpdateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}