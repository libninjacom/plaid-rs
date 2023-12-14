
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportInvestments {
    pub account_id: String,
    pub amount: f64,
    pub date: chrono::NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees: Option<f64>,
    pub investment_transaction_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    pub name: String,
    pub price: f64,
    pub quantity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,
    pub subtype: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for AssetReportInvestments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}