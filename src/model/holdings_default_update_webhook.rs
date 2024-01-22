use serde::{Serialize, Deserialize};
use super::PlaidError;
///Fired when new or updated holdings have been detected on an investment account. The webhook typically fires in response to any newly added holdings or price changes to existing holdings, most commonly after market close.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HoldingsDefaultUpdateWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    ///The number of new holdings reported since the last time this webhook was fired.
    pub new_holdings: f64,
    ///The number of updated holdings reported since the last time this webhook was fired.
    pub updated_holdings: f64,
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    ///`HOLDINGS`
    pub webhook_type: String,
}
impl std::fmt::Display for HoldingsDefaultUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}