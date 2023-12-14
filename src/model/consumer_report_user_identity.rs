
use serde::{Serialize, Deserialize};
use super::AddressData;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerReportUserIdentity {
    pub emails: Vec<String>,
    pub first_name: String,
    pub last_name: String,
    pub phone_numbers: Vec<String>,
    pub primary_address: AddressData,
}
impl std::fmt::Display for ConsumerReportUserIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}