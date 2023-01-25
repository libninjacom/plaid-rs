
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxPaymentProfileResetLoginResponse {
    pub request_id: String,
    pub reset_login: bool,
}
impl std::fmt::Display for SandboxPaymentProfileResetLoginResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}