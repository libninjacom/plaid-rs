use serde::{Serialize, Deserialize};
///Phone number information associated with the entity screening hit
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitPhoneNumbers {
    ///A phone number in E.164 format.
    pub phone_number: String,
    ///An enum indicating whether a phone number is a phone line or a fax line.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for EntityScreeningHitPhoneNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}