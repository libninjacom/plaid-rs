use serde::{Serialize, Deserialize};
///Fired when new bank transfer events are available. Receiving this webhook indicates you should fetch the new events from `/bank_transfer/event/sync`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransfersEventsUpdateWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///`BANK_TRANSFERS_EVENTS_UPDATE`
    pub webhook_code: String,
    ///`BANK_TRANSFERS`
    pub webhook_type: String,
}
impl std::fmt::Display for BankTransfersEventsUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}