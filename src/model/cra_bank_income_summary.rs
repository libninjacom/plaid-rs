
use serde::{Serialize, Deserialize};
use super::{CraBankIncomeHistoricalSummary, CreditAmountWithCurrency};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBankIncomeSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecasted_average_monthly_income: Option<Vec<CreditAmountWithCurrency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_average_monthly_gross_income: Option<Vec<CreditAmountWithCurrency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_average_monthly_income: Option<Vec<CreditAmountWithCurrency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_summary: Option<Vec<CraBankIncomeHistoricalSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_categories_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_sources_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_transactions_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amounts: Option<Vec<CreditAmountWithCurrency>>,
}
impl std::fmt::Display for CraBankIncomeSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}