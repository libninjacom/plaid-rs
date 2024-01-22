use serde::{Serialize, Deserialize};
use super::{CreditBankStatementUploadObject, PayrollItemStatus};
///An object containing information about the bank statement upload Item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankStatementUploadItem {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bank_statements: Vec<CreditBankStatementUploadObject>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    ///Details about the status of the payroll item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PayrollItemStatus>,
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DDTHH:mm:ssZ) indicating the last time that the Item was updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for CreditBankStatementUploadItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}