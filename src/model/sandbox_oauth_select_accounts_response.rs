use serde::{Serialize, Deserialize};
///Defines the response schema for `/sandbox/oauth/select_accounts`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxOauthSelectAccountsResponse {}
impl std::fmt::Display for SandboxOauthSelectAccountsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}