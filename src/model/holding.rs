
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Holding {
    pub account_id: String,
    pub cost_basis: Option<f64>,
    pub institution_price: f64,
    pub institution_price_as_of: Option<String>,
    pub institution_price_datetime: Option<String>,
    pub institution_value: f64,
    pub iso_currency_code: Option<String>,
    pub quantity: f64,
    pub security_id: String,
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for Holding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}