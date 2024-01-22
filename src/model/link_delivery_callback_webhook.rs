use serde::{Serialize, Deserialize};
use super::{LinkCallbackMetadata, PlaidError};
///Webhook containing metadata proxied over from Link callback e.g `onEvent`, `onExit`, `onSuccess`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkDeliveryCallbackWebhook {
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///Information related to the callback from the Hosted Link session.
    pub link_callback_metadata: LinkCallbackMetadata,
    ///The ID of the Hosted Link session.
    pub link_delivery_session_id: String,
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub timestamp: String,
    ///`LINK_CALLBACK`
    pub webhook_code: String,
    ///`LINK_DELIVERY`
    pub webhook_type: String,
}
impl std::fmt::Display for LinkDeliveryCallbackWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}