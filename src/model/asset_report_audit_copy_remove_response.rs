use serde::{Serialize, Deserialize};
///AssetReportAuditCopyRemoveResponse defines the response schema for `/asset_report/audit_copy/remove`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportAuditCopyRemoveResponse {
    ///`true` if the Audit Copy was successfully removed.
    pub removed: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for AssetReportAuditCopyRemoveResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}