use serde::{Serialize, Deserialize};
///An object representing the total deductions for the pay period
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeductionsTotal {
    ///Raw amount of the deduction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<f64>,
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
    ///The year-to-date total amount of the deductions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for DeductionsTotal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}