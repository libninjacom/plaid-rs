
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemPublicTokenCreateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
    pub public_token: String,
    pub request_id: String,
}
impl std::fmt::Display for ItemPublicTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}