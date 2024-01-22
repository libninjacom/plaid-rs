use serde::{Serialize, Deserialize};
///SandboxItemFireWebhookResponse defines the response schema for `/sandbox/item/fire_webhook`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxItemFireWebhookResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///Value is `true`  if the test` webhook_code`  was successfully fired.
    pub webhook_fired: bool,
}
impl std::fmt::Display for SandboxItemFireWebhookResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}