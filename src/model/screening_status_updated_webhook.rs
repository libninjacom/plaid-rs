
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreeningStatusUpdatedWebhook {
    pub environment: String,
    pub screening_id: serde_json::Value,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for ScreeningStatusUpdatedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}