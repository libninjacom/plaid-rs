
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferOriginatorAddress {
    pub city: String,
    pub country_code: String,
    pub postal_code: String,
    pub region: String,
    pub street: String,
}
impl std::fmt::Display for TransferOriginatorAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}