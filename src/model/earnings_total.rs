
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EarningsTotal {
    pub current_amount: Option<f64>,
    pub current_pay: Option<Pay>,
    pub hours: Option<f64>,
    pub iso_currency_code: Option<String>,
    pub unofficial_currency_code: Option<String>,
    pub ytd_amount: Option<f64>,
    pub ytd_pay: Option<Pay>,
}
impl std::fmt::Display for EarningsTotal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}