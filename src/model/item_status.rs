
use serde::{Serialize, Deserialize};
use super::{ItemStatusInvestments, ItemStatusLastWebhook, ItemStatusTransactions};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub investments: Option<ItemStatusInvestments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_webhook: Option<ItemStatusLastWebhook>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<ItemStatusTransactions>,
}
impl std::fmt::Display for ItemStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}