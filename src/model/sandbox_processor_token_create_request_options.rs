
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxProcessorTokenCreateRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_username: Option<String>,
}
impl std::fmt::Display for SandboxProcessorTokenCreateRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}