use serde::{Serialize, Deserialize};
use super::IdentityVerification;
///Paginated list of Plaid sessions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationListResponse {
    ///List of Plaid sessions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identity_verifications: Vec<IdentityVerification>,
    ///An identifier that determines which page of results you receive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for IdentityVerificationListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}