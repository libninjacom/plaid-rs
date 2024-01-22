use serde::{Serialize, Deserialize};
use super::PlaidError;
///Fired when an Item was not verified via automated micro-deposits after seven days since the automated micro-deposit was made.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VerificationExpiredWebhook {
    ///The `account_id` of the account associated with the webhook
    pub account_id: String,
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    ///`VERIFICATION_EXPIRED`
    pub webhook_code: String,
    ///`AUTH`
    pub webhook_type: String,
}
impl std::fmt::Display for VerificationExpiredWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}