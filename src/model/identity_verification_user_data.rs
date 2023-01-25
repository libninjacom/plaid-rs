
use serde::{Serialize, Deserialize};
use super::{IdentityVerificationUserAddress, UserIdNumber, UserName};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationUserData {
    pub address: Option<IdentityVerificationUserAddress>,
    pub date_of_birth: Option<String>,
    pub email_address: Option<String>,
    pub id_number: Option<UserIdNumber>,
    pub ip_address: Option<String>,
    pub name: Option<UserName>,
    pub phone_number: Option<String>,
}
impl std::fmt::Display for IdentityVerificationUserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}