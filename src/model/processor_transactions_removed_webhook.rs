
use serde::{Serialize, Deserialize};
use super::PlaidError;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorTransactionsRemovedWebhook {
    pub account_id: String,
    pub environment: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    pub removed_transactions: Vec<String>,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for ProcessorTransactionsRemovedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}