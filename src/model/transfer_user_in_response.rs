
use serde::{Serialize, Deserialize};
use super::TransferUserAddressInResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferUserInResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<TransferUserAddressInResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    pub legal_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl std::fmt::Display for TransferUserInResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}