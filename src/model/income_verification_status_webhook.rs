
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationStatusWebhook {
    pub environment: String,
    pub item_id: String,
    pub user_id: Option<String>,
    pub verification_status: String,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for IncomeVerificationStatusWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}