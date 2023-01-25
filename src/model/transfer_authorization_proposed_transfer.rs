
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAuthorizationProposedTransfer {
    pub account_id: Option<String>,
    pub ach_class: Option<String>,
    pub amount: String,
    pub iso_currency_code: String,
    pub network: String,
    pub origination_account_id: String,
    pub originator_client_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub user: TransferUserInResponse,
}
impl std::fmt::Display for TransferAuthorizationProposedTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}