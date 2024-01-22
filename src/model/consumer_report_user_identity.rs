use serde::{Serialize, Deserialize};
use super::AddressData;
///ConsumerReportUserIdentity defines the user identity data collected for consumer report purpose. This field is required to be set if you later use the created user for consumer report purpose.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConsumerReportUserIdentity {
    ///The user's emails
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<String>,
    ///The user's first name
    pub first_name: String,
    ///The user's last name
    pub last_name: String,
    ///The user's phone numbers. The format of phone number will be validated and for better normalization, it is expected to be in E.164 format +{countrycode}{number}, for example `+14151234567`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub phone_numbers: Vec<String>,
    ///Data about the components comprising an address.
    pub primary_address: AddressData,
}
impl std::fmt::Display for ConsumerReportUserIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}