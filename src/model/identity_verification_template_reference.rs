use serde::{Serialize, Deserialize};
///The resource ID and version number of the template configuring the behavior of a given Identity Verification.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationTemplateReference {
    ///ID of the associated Identity Verification template.
    pub id: String,
    ///Version of the associated Identity Verification template.
    pub version: i64,
}
impl std::fmt::Display for IdentityVerificationTemplateReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}