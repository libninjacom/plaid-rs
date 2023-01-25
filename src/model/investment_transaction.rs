
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvestmentTransaction {
    pub account_id: String,
    pub amount: f64,
    pub cancel_transaction_id: Option<String>,
    pub date: String,
    pub fees: Option<f64>,
    pub investment_transaction_id: String,
    pub iso_currency_code: Option<String>,
    pub name: String,
    pub price: f64,
    pub quantity: f64,
    pub security_id: Option<String>,
    pub subtype: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for InvestmentTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}