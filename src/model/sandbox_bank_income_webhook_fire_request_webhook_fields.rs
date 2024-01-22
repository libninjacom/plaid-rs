use serde::{Serialize, Deserialize};
///Optional fields which will be populated in the simulated webhook
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxBankIncomeWebhookFireRequestWebhookFields {
    /**The result of the bank income refresh report generation

`SUCCESS`: The refreshed report was successfully generated and can be retrieved via `/credit/bank_income/get`.

`FAILURE`: The refreshed report failed to be generated*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_income_refresh_complete_result: Option<String>,
    ///The user id to be returned in INCOME webhooks
    pub user_id: String,
}
impl std::fmt::Display for SandboxBankIncomeWebhookFireRequestWebhookFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}