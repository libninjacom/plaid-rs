use serde::{Serialize, Deserialize};
use super::PlaidError;
///The `USER_ACCOUNT_REVOKED` webhook is fired when an end user has revoked access to their account on the Data Provider's portal. The user can restore access to the revoked account by regranting permissions on the Data Provider's portal. This webhook is currently in beta. It will be available in GA in Jan 2024.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserAccountRevokedWebhook {
    ///The external account ID of the affected account
    pub account_id: String,
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    ///`USER_ACCOUNT_REVOKED`
    pub webhook_code: String,
    ///`ITEM`
    pub webhook_type: String,
}
impl std::fmt::Display for UserAccountRevokedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}