
use serde::{Serialize, Deserialize};
use super::CreditBankStatementUploadAccountOwnerAddress;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankStatementUploadAccountOwner {
    pub address: CreditBankStatementUploadAccountOwnerAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for CreditBankStatementUploadAccountOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}