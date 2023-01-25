
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Security {
    pub close_price: Option<f64>,
    pub close_price_as_of: Option<String>,
    pub cusip: Option<String>,
    pub institution_id: Option<String>,
    pub institution_security_id: Option<String>,
    pub is_cash_equivalent: Option<bool>,
    pub isin: Option<String>,
    pub iso_currency_code: Option<String>,
    pub name: Option<String>,
    pub proxy_security_id: Option<String>,
    pub security_id: String,
    pub sedol: Option<String>,
    pub ticker_symbol: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub unofficial_currency_code: Option<String>,
    pub update_datetime: Option<String>,
}
impl std::fmt::Display for Security {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}