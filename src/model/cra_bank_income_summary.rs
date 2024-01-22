use serde::{Serialize, Deserialize};
use super::{CraBankIncomeHistoricalSummary, CreditAmountWithCurrency};
///Summary for bank income across all income sources and items (max history of 730 days).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBankIncomeSummary {
    /**The latest date in which all income sources identified by Plaid appear in the user's account.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    ///The predicted average monthly income amount for the income source(s).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forecasted_average_monthly_income: Option<Vec<CreditAmountWithCurrency>>,
    ///An estimate of the average gross monthly income based on the historical net amount and income category for the income source(s).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub historical_average_monthly_gross_income: Option<Vec<CreditAmountWithCurrency>>,
    ///The average monthly income amount estimated based on the historical data for the income source(s).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub historical_average_monthly_income: Option<Vec<CreditAmountWithCurrency>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub historical_summary: Option<Vec<CraBankIncomeHistoricalSummary>>,
    ///Number of income categories per end user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_categories_count: Option<i64>,
    ///Number of income sources per end user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_sources_count: Option<i64>,
    ///Number of income transactions per end user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_transactions_count: Option<i64>,
    /**The earliest date within the days requested in which all income sources identified by Plaid appear in a user's account.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    /**Total amount of earnings across all the income sources in the end user's Items for the days requested by the client.
This can contain multiple amounts, with each amount denominated in one unique currency.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_amounts: Option<Vec<CreditAmountWithCurrency>>,
}
impl std::fmt::Display for CraBankIncomeSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}