
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FdxNotification {
    pub category: String,
    #[serde(rename = "notificationId")]
    pub notification_id: String,
    #[serde(rename = "notificationPayload")]
    pub notification_payload: FdxNotificationPayload,
    pub priority: Option<String>,
    pub publisher: FdxParty,
    #[serde(rename = "sentOn")]
    pub sent_on: String,
    pub severity: Option<String>,
    pub subscriber: Option<FdxParty>,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: Option<FdxHateoasLink>,
}
impl std::fmt::Display for FdxNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}