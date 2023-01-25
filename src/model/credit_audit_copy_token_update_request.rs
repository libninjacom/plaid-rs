
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditAuditCopyTokenUpdateRequest {
    pub audit_copy_token: String,
    pub report_tokens: Vec<String>,
}
impl std::fmt::Display for CreditAuditCopyTokenUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}