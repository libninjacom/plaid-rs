
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollIncomeAccountData {
    pub account_id: Option<String>,
    pub pay_frequency: Option<String>,
    pub rate_of_pay: PayrollIncomeRateOfPay,
}
impl std::fmt::Display for PayrollIncomeAccountData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}