
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HostedMmdVerificationWebhook {
    pub account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    pub item_id: String,
    pub status: String,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for HostedMmdVerificationWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}