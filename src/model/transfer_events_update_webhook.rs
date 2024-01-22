use serde::{Serialize, Deserialize};
///Fired when new transfer events are available. Receiving this webhook indicates you should fetch the new events from `/transfer/event/sync`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferEventsUpdateWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///`TRANSFER_EVENTS_UPDATE`
    pub webhook_code: String,
    ///`TRANSFER`
    pub webhook_type: String,
}
impl std::fmt::Display for TransferEventsUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}