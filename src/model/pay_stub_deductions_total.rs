
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayStubDeductionsTotal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for PayStubDeductionsTotal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}