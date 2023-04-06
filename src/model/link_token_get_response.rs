
use serde::{Serialize, Deserialize};
use super::LinkTokenGetMetadataResponse;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTokenGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
    pub link_token: String,
    pub metadata: LinkTokenGetMetadataResponse,
    pub request_id: String,
}
impl std::fmt::Display for LinkTokenGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}