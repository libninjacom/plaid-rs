
use serde::{Serialize, Deserialize};
use super::Item;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemWebhookUpdateResponse {
    pub item: Item,
    pub request_id: String,
}
impl std::fmt::Display for ItemWebhookUpdateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}