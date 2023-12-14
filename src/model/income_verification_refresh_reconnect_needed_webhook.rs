
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationRefreshReconnectNeededWebhook {
    pub environment: String,
    pub user_id: String,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for IncomeVerificationRefreshReconnectNeededWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}