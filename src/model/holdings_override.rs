use serde::{Serialize, Deserialize};
use super::SecurityOverride;
///Specify the holdings on the account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HoldingsOverride {
    ///The total cost basis of the holding (e.g., the total amount spent to acquire all assets currently in the holding).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_basis: Option<f64>,
    ///Either a valid `iso_currency_code` or `unofficial_currency_code`
    pub currency: String,
    ///The last price given by the institution for this security
    pub institution_price: f64,
    ///The date at which `institution_price` was current. Must be formatted as an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_price_as_of: Option<chrono::NaiveDate>,
    ///The total quantity of the asset held, as reported by the financial institution.
    pub quantity: f64,
    ///Specify the security associated with the holding or investment transaction. When inputting custom security data to the Sandbox, Plaid will perform post-data-retrieval normalization and enrichment. These processes may cause the data returned by the Sandbox to be slightly different from the data you input. An ISO-4217 currency code and a security identifier (`ticker_symbol`, `cusip`, `isin`, or `sedol`) are required.
    pub security: SecurityOverride,
}
impl std::fmt::Display for HoldingsOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}