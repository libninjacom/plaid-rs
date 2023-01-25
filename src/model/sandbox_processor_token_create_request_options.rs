
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxProcessorTokenCreateRequestOptions {
    pub override_password: Option<String>,
    pub override_username: Option<String>,
}
impl std::fmt::Display for SandboxProcessorTokenCreateRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}