use serde::{Serialize, Deserialize};
use super::LinkEvent;
///Contains a summary of the events from a link session
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkEventsWebhook {
    ///The link events emitted during the link session
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<LinkEvent>,
    ///An identifier for the link session these events occurred in
    pub link_session_id: String,
    ///The link token used to create the link session these events are from
    pub link_token: String,
    ///`EVENTS`
    pub webhook_code: String,
    ///`LINK`
    pub webhook_type: String,
}
impl std::fmt::Display for LinkEventsWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}