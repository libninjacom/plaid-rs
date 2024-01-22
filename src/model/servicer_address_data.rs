use serde::{Serialize, Deserialize};
///The address of the student loan servicer. This is generally the remittance address to which payments should be sent.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServicerAddressData {
    ///The full city name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    ///The ISO 3166-1 alpha-2 country code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    ///The postal code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /**The region or state
Example: `"NC"`*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
}
impl std::fmt::Display for ServicerAddressData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}