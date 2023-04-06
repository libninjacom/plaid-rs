
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditAuditCopyTokenUpdateResponse {
    pub request_id: String,
    pub updated: bool,
}
impl std::fmt::Display for CreditAuditCopyTokenUpdateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}