
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRefund {
    pub amount: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub id: String,
    pub status: String,
    pub transfer_id: String,
}
impl std::fmt::Display for TransferRefund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}