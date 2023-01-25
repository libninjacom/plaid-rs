
use serde::{Serialize, Deserialize};
use super::{Email, PhoneNumber, Address};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Owner {
    pub addresses: Vec<Address>,
    pub emails: Vec<Email>,
    pub names: Vec<String>,
    pub phone_numbers: Vec<PhoneNumber>,
}
impl std::fmt::Display for Owner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}