
use serde::{Serialize, Deserialize};
use super::JwkPublicKey;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookVerificationKeyGetResponse {
    pub key: JwkPublicKey,
    pub request_id: String,
}
impl std::fmt::Display for WebhookVerificationKeyGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}