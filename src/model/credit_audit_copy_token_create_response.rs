use serde::{Serialize, Deserialize};
///CreditAuditCopyTokenCreateResponse defines the response schema for `/credit/audit_copy_token/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditAuditCopyTokenCreateResponse {
    ///A token that can be shared with a third party auditor, which allows them to fetch the Asset Reports attached to the token. This token should be stored securely.
    pub audit_copy_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditAuditCopyTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}