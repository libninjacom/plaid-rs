
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestUser {
    pub address: Option<UserAddress>,
    pub client_user_id: String,
    pub date_of_birth: Option<String>,
    pub email_address: Option<String>,
    pub email_address_verified_time: Option<String>,
    pub id_number: Option<UserIdNumber>,
    pub legal_name: Option<String>,
    pub name: Option<serde_json::Value>,
    pub phone_number: Option<String>,
    pub phone_number_verified_time: Option<String>,
    pub ssn: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequestUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}