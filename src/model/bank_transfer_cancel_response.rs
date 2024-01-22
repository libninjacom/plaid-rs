use serde::{Serialize, Deserialize};
///Defines the response schema for `/bank_transfer/cancel`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferCancelResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BankTransferCancelResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}