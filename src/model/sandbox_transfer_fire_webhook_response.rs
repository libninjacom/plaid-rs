use serde::{Serialize, Deserialize};
///Defines the response schema for `/sandbox/transfer/fire_webhook`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxTransferFireWebhookResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SandboxTransferFireWebhookResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}