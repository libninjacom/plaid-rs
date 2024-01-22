use serde::{Serialize, Deserialize};
///Fires when an account has an expired verification when using micro-deposits
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FallbackAuthMicrodepositVerificationExpiredWebhook {
    ///The external account ID associated with the micro-deposit
    pub account_id: String,
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///The error code associated with the webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    ///`VERIFICATION_EXPIRED`
    pub webhook_code: String,
    ///`AUTH`
    pub webhook_type: String,
}
impl std::fmt::Display for FallbackAuthMicrodepositVerificationExpiredWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}