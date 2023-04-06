
use serde::{Serialize, Deserialize};
use super::{SignalAddressData, SignalPersonName};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<SignalAddressData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<SignalPersonName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl std::fmt::Display for SignalUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}