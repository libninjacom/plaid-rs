
use serde::{Serialize, Deserialize};
use super::{BeaconUserIdNumber, BeaconUserName, BeaconUserRequestAddress};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconUserRequestData {
    pub address: BeaconUserRequestAddress,
    pub date_of_birth: chrono::NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<BeaconUserIdNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    pub name: BeaconUserName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl std::fmt::Display for BeaconUserRequestData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}