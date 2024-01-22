use serde::{Serialize, Deserialize};
///The user's address.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositSwitchAddressData {
    ///The full city name
    pub city: String,
    ///The ISO 3166-1 alpha-2 country code
    pub country: String,
    ///The postal code
    pub postal_code: String,
    /**The region or state
Example: `"NC"`*/
    pub region: String,
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: String,
}
impl std::fmt::Display for DepositSwitchAddressData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}