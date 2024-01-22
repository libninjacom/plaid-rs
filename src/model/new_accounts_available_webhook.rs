use serde::{Serialize, Deserialize};
use super::PlaidError;
///Fired when Plaid detects a new account for Items created or updated with [Account Select v2](https://plaid.com/docs/link/customization/#account-select). Upon receiving this webhook, you can prompt your users to share new accounts with you through [Account Select v2 update mode](https://plaid.com/docs/link/update-mode/#using-update-mode-to-request-new-accounts).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NewAccountsAvailableWebhook {
    ///The Plaid environment the webhook was sent from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    ///`NEW_ACCOUNTS_AVAILABLE`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_code: Option<String>,
    ///`ITEM`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_type: Option<String>,
}
impl std::fmt::Display for NewAccountsAvailableWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}