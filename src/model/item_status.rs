use serde::{Serialize, Deserialize};
use super::{ItemStatusInvestments, ItemStatusLastWebhook, ItemStatusTransactions};
///An object with information about the status of the Item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemStatus {
    ///Information about the last successful and failed investments update for the Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub investments: Option<ItemStatusInvestments>,
    ///Information about the last webhook fired for the Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_webhook: Option<ItemStatusLastWebhook>,
    ///Information about the last successful and failed transactions update for the Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transactions: Option<ItemStatusTransactions>,
}
impl std::fmt::Display for ItemStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}