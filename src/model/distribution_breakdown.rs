
use serde::{Serialize, Deserialize};
use super::Pay;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DistributionBreakdown {
    pub account_name: Option<String>,
    pub bank_name: Option<String>,
    pub current_amount: Option<f64>,
    pub current_pay: Option<Pay>,
    pub iso_currency_code: Option<String>,
    pub mask: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for DistributionBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}