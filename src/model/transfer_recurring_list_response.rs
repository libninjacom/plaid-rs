use serde::{Serialize, Deserialize};
use super::RecurringTransfer;
///Defines the response schema for `/transfer/recurring/list`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRecurringListResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recurring_transfers: Vec<RecurringTransfer>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferRecurringListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}