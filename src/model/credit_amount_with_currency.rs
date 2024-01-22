use serde::{Serialize, Deserialize};
///This contains an amount, denominated in the currency specified by either `iso_currency_code` or `unofficial_currency_code`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditAmountWithCurrency {
    ///Value of amount with up to 2 decimal places.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    ///The ISO 4217 currency code of the amount or balance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the amount or balance. Always `null` if `iso_currency_code` is non-null.
Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for CreditAmountWithCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}