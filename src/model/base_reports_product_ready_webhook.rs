use serde::{Serialize, Deserialize};
///Fired when the Base Report has been generated and `/cra/base_report/get` is ready to be called.  If you attempt to retrieve a Base Report before this webhook has fired, you’ll receive a response with the HTTP status code 400 and a Plaid error code of `PRODUCT_NOT_READY`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseReportsProductReadyWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///The `user_id` corresponding to the User ID the webhook has fired for.
    pub user_id: String,
    ///`PRODUCT_READY`
    pub webhook_code: String,
    ///`BASE_REPORT`
    pub webhook_type: String,
}
impl std::fmt::Display for BaseReportsProductReadyWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}