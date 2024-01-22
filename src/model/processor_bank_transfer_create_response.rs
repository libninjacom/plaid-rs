use serde::{Serialize, Deserialize};
use super::BankTransfer;
///Defines the response schema for `/processor/bank_transfer/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorBankTransferCreateResponse {
    ///Represents a bank transfer within the Bank Transfers API.
    pub bank_transfer: BankTransfer,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ProcessorBankTransferCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}