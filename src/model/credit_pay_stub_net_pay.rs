use serde::{Serialize, Deserialize};
///An object representing information about the net pay amount on the pay stub.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditPayStubNetPay {
    ///Raw amount of the net pay for the pay period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<f64>,
    ///Description of the net pay.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the net pay. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
    ///The year-to-date amount of the net pay.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for CreditPayStubNetPay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}