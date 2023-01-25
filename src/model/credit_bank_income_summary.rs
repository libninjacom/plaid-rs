
use serde::{Serialize, Deserialize};
use super::{CreditAmountWithCurrency, CreditBankIncomeHistoricalSummary};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeSummary {
    pub end_date: Option<String>,
    pub historical_summary: Option<Vec<CreditBankIncomeHistoricalSummary>>,
    pub income_categories_count: Option<i64>,
    pub income_sources_count: Option<i64>,
    pub income_transactions_count: Option<i64>,
    pub iso_currency_code: Option<String>,
    pub start_date: Option<String>,
    pub total_amount: Option<f64>,
    pub total_amounts: Option<Vec<CreditAmountWithCurrency>>,
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for CreditBankIncomeSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}