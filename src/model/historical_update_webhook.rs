use serde::{Serialize, Deserialize};
use super::PlaidError;
/**Fired when an Item's historical transaction pull is completed and Plaid has prepared as much historical transaction data as possible for the Item. Once this webhook has been fired, transaction data beyond the most recent 30 days can be fetched for the Item. If [Account Select v2](https://plaid.com/docs/link/customization/#account-select) is enabled, this webhook will also be fired if account selections for the Item are updated, with `new_transactions` set to the number of net new transactions pulled after the account selection update.

This webhook is intended for use with `/transactions/get`; if you are using the newer `/transactions/sync` endpoint, this webhook will still be fired to maintain backwards compatibility, but it is recommended to listen for and respond to the `SYNC_UPDATES_AVAILABLE` webhook instead.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistoricalUpdateWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    ///The number of new, unfetched transactions available
    pub new_transactions: f64,
    ///`HISTORICAL_UPDATE`
    pub webhook_code: String,
    ///`TRANSACTIONS`
    pub webhook_type: String,
}
impl std::fmt::Display for HistoricalUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}