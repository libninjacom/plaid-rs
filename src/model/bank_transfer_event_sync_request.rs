
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferEventSyncRequest {
    pub after_id: i64,
    pub count: Option<i64>,
}
impl std::fmt::Display for BankTransferEventSyncRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}