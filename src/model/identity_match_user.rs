
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityMatchUser {
    pub address: AddressDataNullable,
    pub email_address: Option<String>,
    pub legal_name: Option<String>,
    pub phone_number: Option<String>,
}
impl std::fmt::Display for IdentityMatchUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}