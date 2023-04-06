
use serde::{Serialize, Deserialize};
use super::PayrollIncomeRateOfPay;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollIncomeAccountData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_frequency: Option<String>,
    pub rate_of_pay: PayrollIncomeRateOfPay,
}
impl std::fmt::Display for PayrollIncomeAccountData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}