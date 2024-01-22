use serde::{Serialize, Deserialize};
use super::CreditBankStatementUploadItem;
///CreditBankStatementsUploadsGetResponse defines the response schema for `/credit/bank_statements/uploads/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankStatementsUploadsGetResponse {
    ///Array of bank statement upload items.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CreditBankStatementUploadItem>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditBankStatementsUploadsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}