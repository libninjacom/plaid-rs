
use serde::{Serialize, Deserialize};
use super::Total;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetPay {
    pub current_amount: Option<f64>,
    pub description: Option<String>,
    pub iso_currency_code: Option<String>,
    pub total: Option<Total>,
    pub unofficial_currency_code: Option<String>,
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for NetPay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}