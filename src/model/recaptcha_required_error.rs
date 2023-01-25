
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecaptchaRequiredError {
    pub common_causes: String,
    pub display_message: String,
    pub error_code: String,
    pub error_type: String,
    pub http_code: String,
    pub link_user_experience: String,
    pub troubleshooting_steps: String,
}
impl std::fmt::Display for RecaptchaRequiredError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}