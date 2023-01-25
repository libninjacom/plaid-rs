
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationUserAddress {
    pub city: Option<String>,
    pub country: String,
    pub postal_code: Option<String>,
    pub region: Option<String>,
    pub street: Option<String>,
    pub street2: Option<String>,
}
impl std::fmt::Display for IdentityVerificationUserAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}