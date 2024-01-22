use serde::{Serialize, Deserialize};
///SandboxIncomeFireWebhookResponse defines the response schema for `/sandbox/income/fire_webhook`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxIncomeFireWebhookResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SandboxIncomeFireWebhookResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}