
use serde::{Serialize, Deserialize};
use super::RecurringTransfer;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRecurringGetResponse {
    pub recurring_transfer: RecurringTransfer,
    pub request_id: String,
}
impl std::fmt::Display for TransferRecurringGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}