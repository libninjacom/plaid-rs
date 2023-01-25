
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditPayrollIncomeRefreshResponse {
    pub request_id: String,
    pub verification_refresh_status: String,
}
impl std::fmt::Display for CreditPayrollIncomeRefreshResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}