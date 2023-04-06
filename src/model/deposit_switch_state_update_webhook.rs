
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositSwitchStateUpdateWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_switch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_type: Option<String>,
}
impl std::fmt::Display for DepositSwitchStateUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}