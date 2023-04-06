
use serde::{Serialize, Deserialize};
use super::{RecurringTransferNullable, TransferAuthorizationDecisionRationale};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRecurringCreateResponse {
    pub decision: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_rationale: Option<TransferAuthorizationDecisionRationale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_transfer: Option<RecurringTransferNullable>,
    pub request_id: String,
}
impl std::fmt::Display for TransferRecurringCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}