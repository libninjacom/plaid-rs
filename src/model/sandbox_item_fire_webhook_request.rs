
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxItemFireWebhookRequest {
    pub access_token: String,
    pub webhook_code: String,
    pub webhook_type: Option<String>,
}
impl std::fmt::Display for SandboxItemFireWebhookRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}