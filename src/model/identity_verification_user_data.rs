use serde::{Serialize, Deserialize};
use super::{
    IdentityVerificationResponseUserName, IdentityVerificationUserAddress, UserIdNumber,
};
///The identity data that was either collected from the user or provided via API in order to perform an Identity Verification.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationUserData {
    /**Even if an address has been collected, some fields may be null depending on the region's addressing system. For example:

Addresses from the United Kingdom will not include a region

Addresses from Hong Kong will not include postal code*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<IdentityVerificationUserAddress>,
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    ///A valid email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    ///ID number submitted by the user, currently used only for the Identity Verification product. If the user has not submitted this data yet, this field will be `null`. Otherwise, both fields are guaranteed to be filled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_number: Option<UserIdNumber>,
    ///An IPv4 or IPV6 address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    ///The full name provided by the user. If the user has not submitted their name, this field will be null. Otherwise, both fields are guaranteed to be filled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<IdentityVerificationResponseUserName>,
    ///A phone number in E.164 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl std::fmt::Display for IdentityVerificationUserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}