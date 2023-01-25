
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationRequestUser {
    pub address: Option<UserAddress>,
    pub client_user_id: String,
    pub date_of_birth: Option<String>,
    pub email_address: Option<String>,
    pub id_number: Option<UserIdNumber>,
    pub name: Option<UserName>,
    pub phone_number: Option<String>,
}
impl std::fmt::Display for IdentityVerificationRequestUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}