
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditPayrollIncomeParsingConfigUpdateResponse {
    pub request_id: String,
}
impl std::fmt::Display for CreditPayrollIncomeParsingConfigUpdateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}