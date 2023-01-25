
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemPublicTokenExchangeResponse {
    pub access_token: String,
    pub item_id: String,
    pub request_id: String,
}
impl std::fmt::Display for ItemPublicTokenExchangeResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}