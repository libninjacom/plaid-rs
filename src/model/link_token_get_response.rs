
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTokenGetResponse {
    pub created_at: Option<String>,
    pub expiration: Option<String>,
    pub link_token: String,
    pub metadata: LinkTokenGetMetadataResponse,
    pub request_id: String,
}
impl std::fmt::Display for LinkTokenGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}