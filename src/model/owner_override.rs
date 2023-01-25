
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OwnerOverride {
    pub addresses: Vec<Address>,
    pub emails: Vec<Email>,
    pub names: Vec<String>,
    pub phone_numbers: Vec<PhoneNumber>,
}
impl std::fmt::Display for OwnerOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}