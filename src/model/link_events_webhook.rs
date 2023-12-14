
use serde::{Serialize, Deserialize};
use super::LinkEvent;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkEventsWebhook {
    pub events: Vec<LinkEvent>,
    pub link_session_id: String,
    pub link_token: String,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for LinkEventsWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}