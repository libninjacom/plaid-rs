
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Security {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_price_as_of: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_security_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cash_equivalent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_security_id: Option<String>,
    pub security_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sedol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker_symbol: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_datetime: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for Security {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}