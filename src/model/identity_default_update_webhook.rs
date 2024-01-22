use serde::{Serialize, Deserialize};
use super::{AccountIdsWithUpdatedIdentity, PlaidError};
///Fired when a change to identity data has been detected on an Item. Items are checked for identity updates every 30-90 days. We recommend that upon receiving this webhook you make another call to `/identity/get` to fetch the user's latest identity data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityDefaultUpdateWebhook {
    /**An object with keys of `account_id`'s that are mapped to their respective identity attributes that changed.

Example: `{ "XMBvvyMGQ1UoLbKByoMqH3nXMj84ALSdE5B58": ["PHONES"] }`*/
    pub account_ids_with_updated_identity: AccountIdsWithUpdatedIdentity,
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    ///`IDENTITY`
    pub webhook_type: String,
}
impl std::fmt::Display for IdentityDefaultUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}