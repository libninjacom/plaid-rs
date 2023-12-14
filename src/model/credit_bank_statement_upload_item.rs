
use serde::{Serialize, Deserialize};
use super::{CreditBankStatementUploadObject, PayrollItemStatus};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankStatementUploadItem {
    pub bank_statements: Vec<CreditBankStatementUploadObject>,
    pub item_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PayrollItemStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for CreditBankStatementUploadItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}