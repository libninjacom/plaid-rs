
use serde::{Serialize, Deserialize};
use super::{
    IdentityVerificationResponseUserName, IdentityVerificationUserAddress, UserIdNumber,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationUserData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<IdentityVerificationUserAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<UserIdNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<IdentityVerificationResponseUserName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl std::fmt::Display for IdentityVerificationUserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}