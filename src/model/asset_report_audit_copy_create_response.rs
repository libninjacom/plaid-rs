use serde::{Serialize, Deserialize};
///AssetReportAuditCopyCreateResponse defines the response schema for `/asset_report/audit_copy/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportAuditCopyCreateResponse {
    ///A token that can be shared with a third party auditor to allow them to obtain access to the Asset Report. This token should be stored securely.
    pub audit_copy_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for AssetReportAuditCopyCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}