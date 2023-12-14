
use serde::{Serialize, Deserialize};
use super::{
    CreditBankStatementUploadBankAccount, CreditBankStatementUploadTransaction,
    CreditDocumentMetadata,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankStatementUploadObject {
    pub bank_accounts: Vec<CreditBankStatementUploadBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    pub document_metadata: CreditDocumentMetadata,
    pub transactions: Vec<CreditBankStatementUploadTransaction>,
}
impl std::fmt::Display for CreditBankStatementUploadObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}