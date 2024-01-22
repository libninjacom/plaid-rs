use serde::{Serialize, Deserialize};
///Fired when an Item’s access consent is expiring in 7 days. Some Items have explicit expiration times and we try to relay this when possible to reduce service disruption. This can be resolved by having the user go through Link’s update mode.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PendingExpirationWebhook {
    ///The date and time at which the Item's access consent will expire, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format
    pub consent_expiration_time: chrono::DateTime<chrono::Utc>,
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    ///`PENDING_EXPIRATION`
    pub webhook_code: String,
    ///`ITEM`
    pub webhook_type: String,
}
impl std::fmt::Display for PendingExpirationWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}