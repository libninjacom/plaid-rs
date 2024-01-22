use serde::{Serialize, Deserialize};
use super::{CreditAmountWithCurrency, CreditBankIncomeHistoricalSummary};
///Summary for bank income across all income sources and items (max history of 730 days).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeSummary {
    /**The latest date in which all income sources identified by Plaid appear in the user's account.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub historical_summary: Option<Vec<CreditBankIncomeHistoricalSummary>>,
    ///Number of income categories per end user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_categories_count: Option<i64>,
    ///Number of income sources per end user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_sources_count: Option<i64>,
    ///Number of income transactions per end user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_transactions_count: Option<i64>,
    /**The ISO 4217 currency code of the amount or balance.
Please use [`total_amounts`](https://plaid.com/docs/api/products/income/#credit-bank_income-get-response-bank-income-bank-income-summary-total-amounts) instead.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    /**The earliest date within the days requested in which all income sources identified by Plaid appear in a user's account.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    /**Total amount of earnings across all the income sources in the end user's Items for the days requested by the client.
This may return an incorrect value if the summary includes income sources in multiple currencies.
Please use [`total_amounts`](https://plaid.com/docs/api/products/income/#credit-bank_income-get-response-bank-income-bank-income-summary-total-amounts) instead.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /**Total amount of earnings across all the income sources in the end user's Items for the days requested by the client.
This can contain multiple amounts, with each amount denominated in one unique currency.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_amounts: Option<Vec<CreditAmountWithCurrency>>,
    /**The unofficial currency code associated with the amount or balance. Always `null` if `iso_currency_code` is non-null.
Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.
Please use [`total_amounts`](https://plaid.com/docs/api/products/income/#credit-bank_income-get-response-bank-income-bank-income-summary-total-amounts) instead.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for CreditBankIncomeSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}