use serde::{Serialize, Deserialize};
use super::CreditBankIncomeHistoricalSummary;
///Detailed information for the income source.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeSource {
    ///Plaid's unique identifier for the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /**Maximum of all dates within the specific income sources in the user’s bank account for days requested by the client.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub historical_summary: Option<Vec<CreditBankIncomeHistoricalSummary>>,
    ///The income category. Note that the `CASH` value has been deprecated and is used only for existing legacy implementations. It has been replaced by the new categories `CASH_DEPOSIT` (representing cash or check deposits) and `TRANSFER_FROM_APPLICATION` (representing cash transfers originating from apps, such as Zelle or Venmo).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_category: Option<String>,
    ///The most common name or original description for the underlying income transactions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_description: Option<String>,
    ///A unique identifier for an income source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_source_id: Option<String>,
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
}
impl std::fmt::Display for CreditBankIncomeSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}