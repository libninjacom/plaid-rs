
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkDeliveryGetResponse {
    pub completed_at: Option<String>,
    pub created_at: String,
    pub public_tokens: Option<Vec<String>>,
    pub request_id: String,
    pub status: String,
}
impl std::fmt::Display for LinkDeliveryGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}