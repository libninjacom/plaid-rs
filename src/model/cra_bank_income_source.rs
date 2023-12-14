
use serde::{Serialize, Deserialize};
use super::{CraBankIncomeEmployer, CraBankIncomeHistoricalSummary};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBankIncomeSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employer: Option<CraBankIncomeEmployer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecasted_average_monthly_income: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_average_monthly_gross_income: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_average_monthly_income: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_summary: Option<Vec<CraBankIncomeHistoricalSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_payment_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_frequency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for CraBankIncomeSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}