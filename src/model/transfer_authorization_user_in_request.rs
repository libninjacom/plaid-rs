use serde::{Serialize, Deserialize};
use super::TransferUserAddressInRequest;
///The legal name and other information for the account holder. The `user.legal_name` field is required. Other fields are not currently used and are present to support planned future functionality.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferAuthorizationUserInRequest {
    ///The address associated with the account holder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<TransferUserAddressInRequest>,
    ///The user's email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    ///The user's legal name. If the user is a business, provide the business name.
    pub legal_name: String,
    ///The user's phone number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl std::fmt::Display for TransferAuthorizationUserInRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}