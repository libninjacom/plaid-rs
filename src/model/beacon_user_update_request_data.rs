use serde::{Serialize, Deserialize};
use super::{
    BeaconUserIdNumber, BeaconUserNameNullable, BeaconUserRequestAddressNullable,
};
///A subset of a Beacon User's data which is used to patch the existing identity data associated with a Beacon User. At least one field must be provided,.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconUserUpdateRequestData {
    ///Home address for the associated user. For more context on this field, see [Input Validation by Country](https://plaid.com/docs/identity-verification/hybrid-input-validation/#input-validation-by-country).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<BeaconUserRequestAddressNullable>,
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    ///A valid email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    ///The ID number associated with a Beacon User.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_number: Option<BeaconUserIdNumber>,
    ///An IPv4 or IPV6 address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    ///The full name for a given Beacon User.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<BeaconUserNameNullable>,
    ///A phone number in E.164 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl std::fmt::Display for BeaconUserUpdateRequestData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}