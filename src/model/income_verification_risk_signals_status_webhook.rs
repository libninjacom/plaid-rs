
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationRiskSignalsStatusWebhook {
    pub environment: String,
    pub item_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_signals_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for IncomeVerificationRiskSignalsStatusWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}