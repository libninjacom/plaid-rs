use serde::{Serialize, Deserialize};
use super::AddressData;
///A physical mailing address.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Address {
    ///Data about the components comprising an address.
    pub data: AddressData,
    ///When `true`, identifies the address as the primary address on an account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}
impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}