use serde::{Serialize, Deserialize};
use super::CreditBankStatementUploadAccountOwnerAddress;
///An object containing data about the owner of the bank account for the uploaded bank statement.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankStatementUploadAccountOwner {
    ///Address on the uploaded bank statement
    pub address: CreditBankStatementUploadAccountOwnerAddress,
    ///The name of the account owner
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for CreditBankStatementUploadAccountOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}