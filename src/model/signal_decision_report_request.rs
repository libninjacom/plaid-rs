
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalDecisionReportRequest {
    pub amount_instantly_available: Option<f64>,
    pub client_transaction_id: String,
    pub days_funds_on_hold: Option<i64>,
    pub decision_outcome: Option<String>,
    pub initiated: bool,
    pub payment_method: Option<String>,
}
impl std::fmt::Display for SignalDecisionReportRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}