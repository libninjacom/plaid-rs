use serde::{Serialize, Deserialize};
use super::BankTransfer;
///Defines the response schema for `/bank_transfer/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferGetResponse {
    ///Represents a bank transfer within the Bank Transfers API.
    pub bank_transfer: BankTransfer,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BankTransferGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}