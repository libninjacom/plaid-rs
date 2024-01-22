use serde::{Serialize, Deserialize};
///Address on the paystub
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubAddress {
    ///The full city name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    ///The ISO 3166-1 alpha-2 country code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    ///Street address line 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    ///Street address line 2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    ///The postal code of the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /**The region or state
Example: `"NC"`*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /**The region or state
Example: `"NC"`*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
    ///The full street address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
}
impl std::fmt::Display for PaystubAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}