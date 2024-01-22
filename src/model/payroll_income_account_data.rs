use serde::{Serialize, Deserialize};
use super::PayrollIncomeRateOfPay;
///An object containing account level data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayrollIncomeAccountData {
    ///ID of the payroll provider account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    ///The frequency at which an individual is paid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_frequency: Option<String>,
    ///An object representing the rate at which an individual is paid.
    pub rate_of_pay: PayrollIncomeRateOfPay,
}
impl std::fmt::Display for PayrollIncomeAccountData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}