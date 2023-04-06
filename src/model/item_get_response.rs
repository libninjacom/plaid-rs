
use serde::{Serialize, Deserialize};
use super::{Item, ItemStatus};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemGetResponse {
    pub item: Item,
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}
impl std::fmt::Display for ItemGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}