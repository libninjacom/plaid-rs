
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkSessionFinishedWebhook {
    pub link_session_id: String,
    pub link_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_token: Option<String>,
    pub status: String,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for LinkSessionFinishedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}