use serde::{Serialize, Deserialize};
///CreditAuditCopyTokenRemoveResponse defines the response schema for `/credit/audit_copy_token/remove`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditAuditCopyTokenRemoveResponse {
    ///`true` if the Audit Copy was successfully removed.
    pub removed: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditAuditCopyTokenRemoveResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}