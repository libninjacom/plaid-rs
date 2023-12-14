
use serde::{Serialize, Deserialize};
use super::CreditBankStatementUploadItem;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankStatementsUploadsGetResponse {
    pub items: Vec<CreditBankStatementUploadItem>,
    pub request_id: String,
}
impl std::fmt::Display for CreditBankStatementsUploadsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}