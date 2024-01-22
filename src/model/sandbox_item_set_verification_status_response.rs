use serde::{Serialize, Deserialize};
///SandboxItemSetVerificationStatusResponse defines the response schema for `/sandbox/item/set_verification_status`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxItemSetVerificationStatusResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SandboxItemSetVerificationStatusResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}