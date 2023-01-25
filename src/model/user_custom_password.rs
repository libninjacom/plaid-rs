
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCustomPassword {
    pub force_error: String,
    pub mfa: Mfa,
    pub override_accounts: Vec<OverrideAccounts>,
    pub recaptcha: String,
    pub seed: String,
    pub version: Option<String>,
}
impl std::fmt::Display for UserCustomPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}