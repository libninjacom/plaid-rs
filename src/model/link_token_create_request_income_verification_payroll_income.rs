
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestIncomeVerificationPayrollIncome {
    pub flow_types: Option<Vec<String>>,
    pub is_update_mode: Option<bool>,
}
impl std::fmt::Display for LinkTokenCreateRequestIncomeVerificationPayrollIncome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}