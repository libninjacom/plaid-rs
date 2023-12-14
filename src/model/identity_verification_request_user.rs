
use serde::{Serialize, Deserialize};
use super::{IdentityVerificationRequestUserName, UserAddress, UserIdNumber};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationRequestUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UserAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<UserIdNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<IdentityVerificationRequestUserName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl std::fmt::Display for IdentityVerificationRequestUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}