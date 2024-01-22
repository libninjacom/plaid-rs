use serde::{Serialize, Deserialize};
///Specify the security associated with the holding or investment transaction. When inputting custom security data to the Sandbox, Plaid will perform post-data-retrieval normalization and enrichment. These processes may cause the data returned by the Sandbox to be slightly different from the data you input. An ISO-4217 currency code and a security identifier (`ticker_symbol`, `cusip`, `isin`, or `sedol`) are required.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityOverride {
    ///Either a valid `iso_currency_code` or `unofficial_currency_code`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///9-character CUSIP, an identifier assigned to North American securities. A verified CUSIP Global Services license is required to receive this data. This field will be null by default for new customers, and null for existing customers starting March 12, 2024. If you would like access to this field, please start the verification process [here](https://docs.google.com/forms/d/e/1FAIpQLSd9asHEYEfmf8fxJTHZTAfAzW4dugsnSu-HS2J51f1mxwd6Sw/viewform).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
    ///12-character ISIN, a globally unique securities identifier. A verified CUSIP Global Services license is required to receive this data. This field will be null by default for new customers, and null for existing customers starting March 12, 2024. If you would like access to this field, please start the verification process [here](https://docs.google.com/forms/d/e/1FAIpQLSd9asHEYEfmf8fxJTHZTAfAzW4dugsnSu-HS2J51f1mxwd6Sw/viewform).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub isin: Option<String>,
    ///A descriptive name for the security, suitable for display.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///7-character SEDOL, an identifier assigned to securities in the UK.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sedol: Option<String>,
    ///The security’s trading symbol for publicly traded securities, and otherwise a short identifier if available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticker_symbol: Option<String>,
}
impl std::fmt::Display for SecurityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}