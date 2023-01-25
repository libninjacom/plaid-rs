
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayrollIncomeRateOfPay {
    pub pay_amount: Option<f64>,
    pub pay_rate: Option<String>,
}
impl std::fmt::Display for PayrollIncomeRateOfPay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}