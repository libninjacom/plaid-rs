
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositSwitchAddressData {
    pub city: String,
    pub country: String,
    pub postal_code: String,
    pub region: String,
    pub street: String,
}
impl std::fmt::Display for DepositSwitchAddressData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}