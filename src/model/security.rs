use serde::{Serialize, Deserialize};
use super::OptionContract;
///Contains details about a security
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Security {
    /**Price of the security at the close of the previous trading session. Null for non-public securities.

If the security is a foreign currency this field will be updated daily and will be priced in USD.

If the security is a cryptocurrency, this field will be updated multiple times a day. As crypto prices can fluctuate quickly and data may become stale sooner than other asset classes, refer to `update_datetime` with the time when the price was last updated.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub close_price: Option<f64>,
    ///Date for which `close_price` is accurate. Always `null` if `close_price` is `null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub close_price_as_of: Option<chrono::NaiveDate>,
    ///9-character CUSIP, an identifier assigned to North American securities. A verified CUSIP Global Services license is required to receive this data. This field will be null by default for new customers, and null for existing customers starting March 12, 2024. If you would like access to this field, please start the verification process [here](https://docs.google.com/forms/d/e/1FAIpQLSd9asHEYEfmf8fxJTHZTAfAzW4dugsnSu-HS2J51f1mxwd6Sw/viewform).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
    ///If `institution_security_id` is present, this field indicates the Plaid `institution_id` of the institution to whom the identifier belongs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    ///An identifier given to the security by the institution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_security_id: Option<String>,
    ///Indicates that a security is a highly liquid asset and can be treated like cash.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cash_equivalent: Option<bool>,
    ///12-character ISIN, a globally unique securities identifier. A verified CUSIP Global Services license is required to receive this data. This field will be null by default for new customers, and null for existing customers starting March 12, 2024. If you would like access to this field, please start the verification process [here](https://docs.google.com/forms/d/e/1FAIpQLSd9asHEYEfmf8fxJTHZTAfAzW4dugsnSu-HS2J51f1mxwd6Sw/viewform).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub isin: Option<String>,
    ///The ISO-4217 currency code of the price given. Always `null` if `unofficial_currency_code` is non-`null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    ///The ISO-10383 Market Identifier Code of the exchange or market in which the security is being traded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_identifier_code: Option<String>,
    ///A descriptive name for the security, suitable for display.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /**Details about the option security.

For the Sandbox environment, this data is currently only available if the item is using a custom configuration object, and the `ticker` field of the custom security follows the [OCC Option Symbol](https://en.wikipedia.org/wiki/Option_symbol#The_OCC_Option_Symbol) standard with no spaces.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_contract: Option<OptionContract>,
    ///In certain cases, Plaid will provide the ID of another security whose performance resembles this security, typically when the original security has low volume, or when a private security can be modeled with a publicly traded security.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_security_id: Option<String>,
    ///A unique, Plaid-specific identifier for the security, used to associate securities with holdings. Like all Plaid identifiers, the `security_id` is case sensitive. The `security_id` may change if inherent details of the security change due to a corporate action, for example, in the event of a ticker symbol change or CUSIP change.
    pub security_id: String,
    ///7-character SEDOL, an identifier assigned to securities in the UK.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sedol: Option<String>,
    ///The security’s trading symbol for publicly traded securities, and otherwise a short identifier if available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticker_symbol: Option<String>,
    /**The security type of the holding. Valid security types are:

`cash`: Cash, currency, and money market funds

`cryptocurrency`: Digital or virtual currencies

`derivative`: Options, warrants, and other derivative instruments

`equity`: Domestic and foreign equities

`etf`: Multi-asset exchange-traded investment funds

`fixed income`: Bonds and certificates of deposit (CDs)

`loan`: Loans and loan receivables

`mutual fund`: Open- and closed-end vehicles pooling funds of multiple investors

`other`: Unknown or other investment types*/
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /**The unofficial currency code associated with the security. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
    ///Date and time at which `close_price` is accurate, in ISO 8601 format (YYYY-MM-DDTHH:mm:ssZ). Always `null` if `close_price` is `null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_datetime: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for Security {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}