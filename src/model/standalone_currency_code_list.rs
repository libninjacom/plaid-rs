
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StandaloneCurrencyCodeList {
    pub iso_currency_code: String,
    pub unofficial_currency_code: String,
}
impl std::fmt::Display for StandaloneCurrencyCodeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}