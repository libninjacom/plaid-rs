
use serde::{Serialize, Deserialize};
use super::{CreditAmountWithCurrency, CreditBankIncomeHistoricalSummary};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_summary: Option<Vec<CreditBankIncomeHistoricalSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_categories_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_sources_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_transactions_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amounts: Option<Vec<CreditAmountWithCurrency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for CreditBankIncomeSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}