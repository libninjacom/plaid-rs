use serde::{Serialize, Deserialize};
use super::Pay;
///An object representing both the current pay period and year to date amount for an earning category.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EarningsTotal {
    ///Total amount of the earnings for this pay period
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<f64>,
    ///An object representing a monetary amount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_pay: Option<Pay>,
    ///Total number of hours worked for this pay period
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours: Option<f64>,
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the security. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
    ///The total year-to-date amount of the earnings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ytd_amount: Option<f64>,
    ///An object representing a monetary amount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ytd_pay: Option<Pay>,
}
impl std::fmt::Display for EarningsTotal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}