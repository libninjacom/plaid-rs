
use serde::{Serialize, Deserialize};
use super::CreditBankEmployer;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankEmployment {
    pub account_id: String,
    pub bank_employment_id: String,
    pub earliest_deposit_date: chrono::NaiveDate,
    pub employer: CreditBankEmployer,
    pub latest_deposit_date: chrono::NaiveDate,
}
impl std::fmt::Display for CreditBankEmployment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}