use serde::{Serialize, Deserialize};
/**Details about the option security.

For the Sandbox environment, this data is currently only available if the item is using a custom configuration object, and the `ticker` field of the custom security follows the [OCC Option Symbol](https://en.wikipedia.org/wiki/Option_symbol#The_OCC_Option_Symbol) standard with no spaces.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptionContract {
    /**The type of this option contract. It is one of:

`put`: for Put option contracts

`call`: for Call option contracts*/
    pub contract_type: String,
    ///The expiration date for this option contract, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub expiration_date: chrono::NaiveDate,
    ///The strike price for this option contract, per share of security.
    pub strike_price: f64,
    ///The ticker of the underlying security for this option contract.
    pub underlying_security_ticker: String,
}
impl std::fmt::Display for OptionContract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}