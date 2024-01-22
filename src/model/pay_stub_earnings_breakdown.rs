use serde::{Serialize, Deserialize};
///An object representing the earnings line items for the pay period.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayStubEarningsBreakdown {
    ///Commonly used term to describe the earning line item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonical_description: Option<String>,
    ///Raw amount of the earning line item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<f64>,
    ///Description of the earning line item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Number of hours applicable for this earning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours: Option<f64>,
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    ///Hourly rate applicable for this earning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    /**The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
    ///The year-to-date amount of the deduction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for PayStubEarningsBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}