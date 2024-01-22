use serde::{Serialize, Deserialize};
use super::TransferUserAddressInResponse;
///The legal name and other information for the account holder.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferUserInResponse {
    ///The address associated with the account holder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<TransferUserAddressInResponse>,
    ///The user's email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    ///The user's legal name.
    pub legal_name: String,
    ///The user's phone number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl std::fmt::Display for TransferUserInResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}