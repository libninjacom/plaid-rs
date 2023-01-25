
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxIncomeFireWebhookRequest {
    pub item_id: String,
    pub user_id: Option<String>,
    pub verification_status: String,
    pub webhook: String,
}
impl std::fmt::Display for SandboxIncomeFireWebhookRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}