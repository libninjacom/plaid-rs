
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NewAccountsAvailableWebhook {
    pub environment: Option<String>,
    pub error: Option<PlaidError>,
    pub item_id: Option<String>,
    pub webhook_code: Option<String>,
    pub webhook_type: Option<String>,
}
impl std::fmt::Display for NewAccountsAvailableWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}