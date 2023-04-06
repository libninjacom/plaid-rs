
use serde::{Serialize, Deserialize};
use super::{LinkCallbackMetadata, PlaidError};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkDeliveryCallbackWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    pub link_callback_metadata: LinkCallbackMetadata,
    pub link_delivery_session_id: String,
    pub timestamp: String,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for LinkDeliveryCallbackWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}