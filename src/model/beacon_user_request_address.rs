use serde::{Serialize, Deserialize};
///Home address for the associated user. For more context on this field, see [Input Validation by Country](https://plaid.com/docs/identity-verification/hybrid-input-validation/#input-validation-by-country).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconUserRequestAddress {
    ///City from the end user's address
    pub city: String,
    ///Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    pub country: String,
    ///The postal code for the associated address. Between 2 and 10 alphanumeric characters. For US-based addresses this must be 5 numeric digits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    ///An ISO 3166-2 subdivision code. Related terms would be "state", "province", "prefecture", "zone", "subdivision", etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    ///The primary street portion of an address. If an address is provided, this field will always be filled.
    pub street: String,
    ///Extra street information, like an apartment or suite number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street2: Option<String>,
}
impl std::fmt::Display for BeaconUserRequestAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}