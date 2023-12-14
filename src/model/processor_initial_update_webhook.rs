
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorInitialUpdateWebhook {
    pub account_id: String,
    pub environment: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub new_transactions: f64,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for ProcessorInitialUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}