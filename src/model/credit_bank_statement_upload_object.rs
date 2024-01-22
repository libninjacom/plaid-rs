use serde::{Serialize, Deserialize};
use super::{
    CreditBankStatementUploadBankAccount, CreditBankStatementUploadTransaction,
    CreditDocumentMetadata,
};
///An object containing data that has been parsed from a user-uploaded bank statement.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankStatementUploadObject {
    ///An array of bank accounts associated with the uploaded bank statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bank_accounts: Vec<CreditBankStatementUploadBankAccount>,
    ///An identifier of the document referenced by the document metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///Object representing metadata pertaining to the document.
    pub document_metadata: CreditDocumentMetadata,
    ///An array of transactions appearing on the bank statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transactions: Vec<CreditBankStatementUploadTransaction>,
}
impl std::fmt::Display for CreditBankStatementUploadObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}