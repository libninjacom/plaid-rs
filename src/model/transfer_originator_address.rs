use serde::{Serialize, Deserialize};
///The originator's address.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferOriginatorAddress {
    ///The full city name.
    pub city: String,
    ///ISO-3166-1 alpha-2 country code standard.
    pub country_code: String,
    ///The postal code (e.g., "94103").
    pub postal_code: String,
    ///The two-letter code for the state or province (e.g., "CA").
    pub region: String,
    ///The full street address.
    pub street: String,
}
impl std::fmt::Display for TransferOriginatorAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}