
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxItemResetLoginResponse {
    pub request_id: String,
    pub reset_login: bool,
}
impl std::fmt::Display for SandboxItemResetLoginResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}