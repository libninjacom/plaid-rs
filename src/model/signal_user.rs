use serde::{Serialize, Deserialize};
use super::{SignalAddressData, SignalPersonName};
///Details about the end user initiating the transaction (i.e., the account holder).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalUser {
    ///Data about the components comprising an address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<SignalAddressData>,
    ///The user's email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    ///The user's legal name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<SignalPersonName>,
    ///The user's phone number, in E.164 format: +{countrycode}{number}. For example: "+14151234567"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl std::fmt::Display for SignalUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}