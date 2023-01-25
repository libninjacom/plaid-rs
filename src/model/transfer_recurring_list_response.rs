
use serde::{Serialize, Deserialize};
use super::RecurringTransfer;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRecurringListResponse {
    pub recurring_transfers: Vec<RecurringTransfer>,
    pub request_id: String,
}
impl std::fmt::Display for TransferRecurringListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}