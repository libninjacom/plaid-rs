
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayStubDistributionBreakdown {
    pub account_name: Option<String>,
    pub bank_name: Option<String>,
    pub current_amount: Option<f64>,
    pub iso_currency_code: Option<String>,
    pub mask: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for PayStubDistributionBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}