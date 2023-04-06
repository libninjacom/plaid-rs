
use serde::{Serialize, Deserialize};
use super::LinkDeliveryMetadata;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkUserDeliveryStatusWebhook {
    pub link_delivery_metadata: LinkDeliveryMetadata,
    pub link_delivery_session_id: String,
    pub timestamp: String,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for LinkUserDeliveryStatusWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}