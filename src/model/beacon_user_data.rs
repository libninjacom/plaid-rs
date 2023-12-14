
use serde::{Serialize, Deserialize};
use super::{BeaconUserAddress, BeaconUserIdNumber, BeaconUserName};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconUserData {
    pub address: BeaconUserAddress,
    pub date_of_birth: chrono::NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    pub id_number: BeaconUserIdNumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    pub name: BeaconUserName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl std::fmt::Display for BeaconUserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}