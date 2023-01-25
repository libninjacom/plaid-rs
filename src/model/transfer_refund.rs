
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRefund {
    pub amount: String,
    pub created: String,
    pub id: String,
    pub status: String,
    pub transfer_id: String,
}
impl std::fmt::Display for TransferRefund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}