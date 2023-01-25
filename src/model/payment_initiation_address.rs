
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationAddress {
    pub city: String,
    pub country: String,
    pub postal_code: String,
    pub street: Vec<String>,
}
impl std::fmt::Display for PaymentInitiationAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}