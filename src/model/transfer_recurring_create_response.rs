
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRecurringCreateResponse {
    pub decision: String,
    pub decision_rationale: Option<TransferAuthorizationDecisionRationale>,
    pub recurring_transfer: Option<RecurringTransferNullable>,
    pub request_id: String,
}
impl std::fmt::Display for TransferRecurringCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}