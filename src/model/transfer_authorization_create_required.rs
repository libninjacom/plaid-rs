
use serde::{Serialize, Deserialize};
use super::TransferAuthorizationUserInRequest;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAuthorizationCreateRequired {
    pub amount: String,
    pub network: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub user: TransferAuthorizationUserInRequest,
}
impl std::fmt::Display for TransferAuthorizationCreateRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}