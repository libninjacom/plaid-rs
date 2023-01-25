
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayStubEarningsBreakdown {
    pub canonical_description: Option<String>,
    pub current_amount: Option<f64>,
    pub description: Option<String>,
    pub hours: Option<f64>,
    pub iso_currency_code: Option<String>,
    pub rate: Option<f64>,
    pub unofficial_currency_code: Option<String>,
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for PayStubEarningsBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}