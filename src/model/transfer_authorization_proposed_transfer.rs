
use serde::{Serialize, Deserialize};
use super::{TransferCreditFundsSource, TransferUserInResponse};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAuthorizationProposedTransfer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_class: Option<String>,
    pub amount: String,
    pub credit_funds_source: TransferCreditFundsSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_account_id: Option<String>,
    pub iso_currency_code: String,
    pub network: String,
    pub origination_account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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