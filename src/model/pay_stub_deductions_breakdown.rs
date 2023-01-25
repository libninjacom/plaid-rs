
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayStubDeductionsBreakdown {
    pub current_amount: Option<f64>,
    pub description: Option<String>,
    pub iso_currency_code: Option<String>,
    pub unofficial_currency_code: Option<String>,
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for PayStubDeductionsBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}