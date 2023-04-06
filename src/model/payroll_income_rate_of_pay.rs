
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayrollIncomeRateOfPay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_rate: Option<String>,
}
impl std::fmt::Display for PayrollIncomeRateOfPay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}