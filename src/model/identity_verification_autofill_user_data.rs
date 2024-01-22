use serde::{Serialize, Deserialize};
use super::{
    IdentityVerificationAutofillAddress, IdentityVerificationResponseUserName,
    UserIdNumber,
};
///User information that was autofilled. All this information should be confirmed by the user before using.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationAutofillUserData {
    /**Even if an address has been autofilled, some fields may be null depending on the region's addressing system. For example:

Addresses from the United Kingdom will not include a region

Addresses from Hong Kong will not include postal code*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<IdentityVerificationAutofillAddress>,
    ///ID number submitted by the user, currently used only for the Identity Verification product. If the user has not submitted this data yet, this field will be `null`. Otherwise, both fields are guaranteed to be filled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_number: Option<UserIdNumber>,
    ///The full name provided by the user. If the user has not submitted their name, this field will be null. Otherwise, both fields are guaranteed to be filled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<IdentityVerificationResponseUserName>,
}
impl std::fmt::Display for IdentityVerificationAutofillUserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}