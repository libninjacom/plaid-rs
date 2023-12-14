
use serde::{Serialize, Deserialize};
use super::{
    CreditBankStatementUploadAccountOwner, CreditBankStatementUploadBankAccountPeriod,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankStatementUploadBankAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub owner: CreditBankStatementUploadAccountOwner,
    pub periods: Vec<CreditBankStatementUploadBankAccountPeriod>,
}
impl std::fmt::Display for CreditBankStatementUploadBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}