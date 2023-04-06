
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringTransferSkippedWebhook {
    pub authorization_decision: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_decision_rationale_code: Option<String>,
    pub environment: String,
    pub recurring_transfer_id: String,
    pub skipped_origination_date: chrono::NaiveDate,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for RecurringTransferSkippedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}