use serde::{Serialize, Deserialize};
///A phone number
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PhoneNumber {
    ///The phone number.
    pub data: String,
    ///When `true`, identifies the phone number as the primary number on an account.
    pub primary: bool,
    ///The type of phone number.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}