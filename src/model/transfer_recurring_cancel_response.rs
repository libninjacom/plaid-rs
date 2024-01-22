use serde::{Serialize, Deserialize};
///Defines the response schema for `/transfer/recurring/cancel`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRecurringCancelResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferRecurringCancelResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}