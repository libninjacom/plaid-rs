
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxItemFireWebhookResponse {
    pub request_id: String,
    pub webhook_fired: bool,
}
impl std::fmt::Display for SandboxItemFireWebhookResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}