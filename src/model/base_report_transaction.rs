
use serde::{Serialize, Deserialize};
use super::{CreditCategory, Location};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseReportTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_owner: Option<String>,
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_category: Option<CreditCategory>,
    pub date: chrono::NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_transacted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_description: Option<String>,
    pub pending: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for BaseReportTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}