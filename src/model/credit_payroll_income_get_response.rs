
use serde::{Serialize, Deserialize};
use super::{PayrollItem, PlaidError};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditPayrollIncomeGetResponse {
    pub error: Option<PlaidError>,
    pub items: Vec<PayrollItem>,
    pub request_id: String,
}
impl std::fmt::Display for CreditPayrollIncomeGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}