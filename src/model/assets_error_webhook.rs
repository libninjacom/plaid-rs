use serde::{Serialize, Deserialize};
use super::PlaidError;
///Fired when Asset Report generation has failed. The resulting `error` will have an `error_type` of `ASSET_REPORT_ERROR`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetsErrorWebhook {
    ///The ID associated with the Asset Report.
    pub asset_report_id: String,
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///The `user_id` corresponding to the User ID the webhook has fired for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    ///`ERROR`
    pub webhook_code: String,
    ///`ASSETS`
    pub webhook_type: String,
}
impl std::fmt::Display for AssetsErrorWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}