
use serde::{Serialize, Deserialize};
use super::{UserAddress, UserIdNumber};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UserAddress>,
    pub client_user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address_verified_time: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<UserIdNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_verified_time: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequestUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}