
use serde::{Serialize, Deserialize};
use super::CreditBankIncomeHistoricalSummary;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeSource {
    pub account_id: Option<String>,
    pub end_date: Option<String>,
    pub historical_summary: Option<Vec<CreditBankIncomeHistoricalSummary>>,
    pub income_category: Option<String>,
    pub income_description: Option<String>,
    pub income_source_id: Option<String>,
    pub pay_frequency: Option<String>,
    pub start_date: Option<String>,
    pub total_amount: Option<f64>,
    pub transaction_count: Option<i64>,
}
impl std::fmt::Display for CreditBankIncomeSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}