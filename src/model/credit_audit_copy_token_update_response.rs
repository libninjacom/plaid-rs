use serde::{Serialize, Deserialize};
///CreditAuditCopyTokenUpdateResponse defines the response schema for `/credit/audit_copy_token/update`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditAuditCopyTokenUpdateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///`true` if the Audit Copy Token was successfully updated.
    pub updated: bool,
}
impl std::fmt::Display for CreditAuditCopyTokenUpdateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}