use serde::{Serialize, Deserialize};
use super::{CraBankIncomeEmployer, CraBankIncomeHistoricalSummary};
///Detailed information for the income source.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBankIncomeSource {
    ///The object containing employer data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employer: Option<CraBankIncomeEmployer>,
    /**Maximum of all dates within the specific income sources in the user’s bank account for days requested by the client.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    ///The predicted average monthly net income amount for the income source(s).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forecasted_average_monthly_income: Option<f64>,
    ///An estimate of the average gross monthly income based on the historical net amount and income category for the income source(s).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub historical_average_monthly_gross_income: Option<f64>,
    ///The average monthly net income amount estimated based on the historical data for the income source(s).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub historical_average_monthly_income: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub historical_summary: Option<Vec<CraBankIncomeHistoricalSummary>>,
    ///The income category. Note that the `CASH` value has been deprecated and is used only for existing legacy implementations. It has been replaced by the new categories `CASH_DEPOSIT` (representing cash or check deposits) and `TRANSFER_FROM_APPLICATION` (representing cash transfers originating from apps, such as Zelle or Venmo).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_category: Option<String>,
    ///The most common name or original description for the underlying income transactions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_description: Option<String>,
    ///A unique identifier for an income source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_source_id: Option<String>,
    ///The ISO 4217 currency code of the amount or balance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    /**The expected date of the end user’s next paycheck for the income source.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_payment_date: Option<chrono::NaiveDate>,
    ///The income pay frequency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_frequency: Option<String>,
    /**Minimum of all dates within the specific income sources in the user's bank account for days requested by the client.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    ///Total amount of earnings in the user’s bank account for the specific income source for days requested by the client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    ///Number of transactions for the income source within the start and end date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_count: Option<i64>,
    /**The unofficial currency code associated with the amount or balance. Always `null` if `iso_currency_code` is non-null.
Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for CraBankIncomeSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}