use serde::{Serialize, Deserialize};
use super::LinkDeliveryMetadata;
///Webhook indicating that the status of the delivery of the Hosted Link session to a user
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkUserDeliveryStatusWebhook {
    ///Information related to the related to the delivery of the link session to users
    pub link_delivery_metadata: LinkDeliveryMetadata,
    ///The ID of the Hosted Link session.
    pub link_delivery_session_id: String,
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub timestamp: String,
    ///`DELIVERY_STATUS`
    pub webhook_code: String,
    ///`LINK_DELIVERY`
    pub webhook_type: String,
}
impl std::fmt::Display for LinkUserDeliveryStatusWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}