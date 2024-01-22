use serde::{Serialize, Deserialize};
use super::SecurityOverride;
///Specify the list of investments transactions on the account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvestmentsTransactionsOverride {
    ///Either a valid `iso_currency_code` or `unofficial_currency_code`
    pub currency: String,
    ///Posting date for the transaction. Must be formatted as an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) date.
    pub date: chrono::NaiveDate,
    ///The combined value of all fees applied to this transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fees: Option<f64>,
    ///The institution's description of the transaction.
    pub name: String,
    ///The price of the security at which this transaction occurred.
    pub price: f64,
    ///The number of units of the security involved in this transaction. Must be positive if the type is a buy and negative if the type is a sell.
    pub quantity: f64,
    ///Specify the security associated with the holding or investment transaction. When inputting custom security data to the Sandbox, Plaid will perform post-data-retrieval normalization and enrichment. These processes may cause the data returned by the Sandbox to be slightly different from the data you input. An ISO-4217 currency code and a security identifier (`ticker_symbol`, `cusip`, `isin`, or `sedol`) are required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security: Option<SecurityOverride>,
    /**The type of the investment transaction. Possible values are:
`buy`: Buying an investment
`sell`: Selling an investment
`cash`: Activity that modifies a cash position
`fee`: A fee on the account
`transfer`: Activity that modifies a position, but not through buy/sell activity e.g. options exercise, portfolio transfer*/
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for InvestmentsTransactionsOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}