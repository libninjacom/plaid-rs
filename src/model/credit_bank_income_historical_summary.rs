
use serde::{Serialize, Deserialize};
use super::{CreditAmountWithCurrency, CreditBankIncomeTransaction};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeHistoricalSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amounts: Option<Vec<CreditAmountWithCurrency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<CreditBankIncomeTransaction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for CreditBankIncomeHistoricalSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}