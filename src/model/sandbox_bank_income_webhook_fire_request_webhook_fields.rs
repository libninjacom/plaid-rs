
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxBankIncomeWebhookFireRequestWebhookFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_income_refresh_complete_result: Option<String>,
    pub user_id: String,
}
impl std::fmt::Display for SandboxBankIncomeWebhookFireRequestWebhookFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}