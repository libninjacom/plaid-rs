use serde::{Serialize, Deserialize};
use super::PayStubDistributionBreakdown;
///Details about the pay period.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayStubPayPeriodDetails {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub distribution_breakdown: Vec<PayStubDistributionBreakdown>,
    ///The date on which the pay period ended, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    ///Total earnings before tax/deductions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gross_earnings: Option<f64>,
    ///The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    ///The amount of the paycheck.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_amount: Option<f64>,
    ///The explicit pay basis on the paystub (if present).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_basis: Option<String>,
    ///The date on which the pay stub was issued, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_date: Option<chrono::NaiveDate>,
    ///The frequency at which an individual is paid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_frequency: Option<String>,
    ///The date on which the pay period started, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    /**The unofficial currency code associated with the net pay. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for PayStubPayPeriodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}