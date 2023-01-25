
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditPayrollIncomeRiskSignalsGetResponse {
    pub error: Option<PlaidError>,
    pub items: Vec<PayrollRiskSignalsItem>,
    pub request_id: String,
}
impl std::fmt::Display for CreditPayrollIncomeRiskSignalsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}