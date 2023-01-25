
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxOauthSelectAccountsRequest {
    pub accounts: Vec<String>,
    pub oauth_state_id: String,
}
impl std::fmt::Display for SandboxOauthSelectAccountsRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}