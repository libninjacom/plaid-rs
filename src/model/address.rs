
use serde::{Serialize, Deserialize};
use super::AddressData;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub data: AddressData,
    pub primary: Option<bool>,
}
impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}