
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositSwitchStateUpdateWebhook {
    pub deposit_switch_id: Option<String>,
    pub environment: Option<String>,
    pub state: Option<String>,
    pub webhook_code: Option<String>,
    pub webhook_type: Option<String>,
}
impl std::fmt::Display for DepositSwitchStateUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}