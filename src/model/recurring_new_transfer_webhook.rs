use serde::{Serialize, Deserialize};
///Fired when a new transfer of a recurring transfer is originated.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecurringNewTransferWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///Plaid’s unique identifier for a recurring transfer.
    pub recurring_transfer_id: String,
    ///Plaid’s unique identifier for a transfer.
    pub transfer_id: String,
    ///`RECURRING_NEW_TRANSFER`
    pub webhook_code: String,
    ///`TRANSFER`
    pub webhook_type: String,
}
impl std::fmt::Display for RecurringNewTransferWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}