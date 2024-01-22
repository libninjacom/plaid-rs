use serde::{Serialize, Deserialize};
///SandboxBankIncomeFireWebhookResponse defines the response schema for `/sandbox/bank_income/fire_webhook`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxBankIncomeFireWebhookResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SandboxBankIncomeFireWebhookResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}