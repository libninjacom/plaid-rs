use serde::{Serialize, Deserialize};
use super::RecurringTransfer;
///Defines the response schema for `/transfer/recurring/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRecurringGetResponse {
    ///Represents a recurring transfer within the Transfers API.
    pub recurring_transfer: RecurringTransfer,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferRecurringGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}