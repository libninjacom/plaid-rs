
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalUser {
    pub address: Option<SignalAddressData>,
    pub email_address: Option<String>,
    pub name: Option<SignalPersonName>,
    pub phone_number: Option<String>,
}
impl std::fmt::Display for SignalUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}