
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkDeliveryGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_tokens: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_ids: Option<Vec<String>>,
    pub request_id: String,
    pub status: String,
}
impl std::fmt::Display for LinkDeliveryGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}