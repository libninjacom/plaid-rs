use serde::{Serialize, Deserialize};
use super::AddressDataNotRequired;
///The user's legal name, phone number, email address and address used to perform fuzzy match. If Financial Account Matching is enabled in the Identity Verification product, leave this field empty to automatically match against PII collected from the Identity Verification checks.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityMatchUser {
    ///Data about the components comprising an address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressDataNotRequired>,
    ///The user's email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    ///The user's full legal name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    ///The user's phone number, in E.164 format: +{countrycode}{number}. For example: "+14151234567". Phone numbers provided in other formats will be parsed on a best-effort basis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl std::fmt::Display for IdentityMatchUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}