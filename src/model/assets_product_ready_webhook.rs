use serde::{Serialize, Deserialize};
///Fired when the Asset Report has been generated and `/asset_report/get` is ready to be called.  If you attempt to retrieve an Asset Report before this webhook has fired, you’ll receive a response with the HTTP status code 400 and a Plaid error code of `PRODUCT_NOT_READY`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetsProductReadyWebhook {
    ///The `asset_report_id` corresponding to the Asset Report the webhook has fired for.
    pub asset_report_id: String,
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///The report type, indicating whether the Asset Report is a `full` or `fast` report.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_type: Option<String>,
    ///The `user_id` corresponding to the User ID the webhook has fired for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    ///`PRODUCT_READY`
    pub webhook_code: String,
    ///`ASSETS`
    pub webhook_type: String,
}
impl std::fmt::Display for AssetsProductReadyWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}