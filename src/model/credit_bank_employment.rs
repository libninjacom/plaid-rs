use serde::{Serialize, Deserialize};
use super::CreditBankEmployer;
///Detailed information for the bank employment.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankEmployment {
    ///Plaid's unique identifier for the account.
    pub account_id: String,
    ///A unique identifier for the bank employment.
    pub bank_employment_id: String,
    ///The date of the earliest deposit from this employer from within the period of the days requested.
    pub earliest_deposit_date: chrono::NaiveDate,
    ///Object containing employer data.
    pub employer: CreditBankEmployer,
    ///The date of the most recent deposit from this employer.
    pub latest_deposit_date: chrono::NaiveDate,
}
impl std::fmt::Display for CreditBankEmployment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}