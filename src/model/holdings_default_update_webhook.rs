
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HoldingsDefaultUpdateWebhook {
    pub environment: String,
    pub error: Option<PlaidError>,
    pub item_id: String,
    pub new_holdings: f64,
    pub updated_holdings: f64,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for HoldingsDefaultUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}