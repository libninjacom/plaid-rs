
use serde::{Serialize, Deserialize};
use super::CreditBankIncomeHistoricalSummary;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_summary: Option<Vec<CreditBankIncomeHistoricalSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_frequency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_count: Option<i64>,
}
impl std::fmt::Display for CreditBankIncomeSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}