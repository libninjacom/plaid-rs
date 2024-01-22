use serde::{Serialize, Deserialize};
///Data about the components comprising an address.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddressData {
    ///The full city name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    ///The ISO 3166-1 alpha-2 country code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    ///The postal code. In API versions 2018-05-22 and earlier, this field is called `zip`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /**The region or state. In API versions 2018-05-22 and earlier, this field is called `state`.
Example: `"NC"`*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: String,
}
impl std::fmt::Display for AddressData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}