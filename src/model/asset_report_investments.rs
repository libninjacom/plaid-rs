use serde::{Serialize, Deserialize};
///A transaction within an investment account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportInvestments {
    ///The `account_id` of the account against which this transaction posted.
    pub account_id: String,
    ///The complete value of the transaction. Positive values when cash is debited, e.g. purchases of stock; negative values when cash is credited, e.g. sales of stock. Treatment remains the same for cash-only movements unassociated with securities.
    pub amount: f64,
    ///The [ISO 8601](https://wikipedia.org/wiki/ISO_8601) posting date for the transaction.
    pub date: chrono::NaiveDate,
    ///The combined value of all fees applied to this transaction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fees: Option<f64>,
    ///The ID of the Investment transaction, unique across all Plaid transactions. Like all Plaid identifiers, the `investment_transaction_id` is case sensitive.
    pub investment_transaction_id: String,
    ///The ISO-4217 currency code of the transaction. Always `null` if `unofficial_currency_code` is non-`null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    ///The institution’s description of the transaction.
    pub name: String,
    ///The price of the security at which this transaction occurred.
    pub price: f64,
    ///The number of units of the security involved in this transaction. Positive for buy transactions; negative for sell transactions.
    pub quantity: f64,
    ///The `security_id` to which this transaction is related.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,
    ///For descriptions of possible transaction types and subtypes, see the [Investment transaction types schema](https://plaid.com/docs/api/accounts/#investment-transaction-types-schema).
    pub subtype: String,
    /**Value is one of the following:
`buy`: Buying an investment
`sell`: Selling an investment
`cancel`: A cancellation of a pending transaction
`cash`: Activity that modifies a cash position
`fee`: A fee on the account
`transfer`: Activity which modifies a position, but not through buy/sell activity e.g. options exercise, portfolio transfer

For descriptions of possible transaction types and subtypes, see the [Investment transaction types schema](https://plaid.com/docs/api/accounts/#investment-transaction-types-schema).*/
    #[serde(rename = "type")]
    pub type_: String,
    /**The unofficial currency code associated with the holding. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for AssetReportInvestments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}