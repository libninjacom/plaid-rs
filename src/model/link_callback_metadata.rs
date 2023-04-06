
use serde::{Serialize, Deserialize};
use super::{LinkDeliveryAccount, LinkDeliveryInstitution};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkCallbackMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<LinkDeliveryAccount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution: Option<LinkDeliveryInstitution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for LinkCallbackMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}