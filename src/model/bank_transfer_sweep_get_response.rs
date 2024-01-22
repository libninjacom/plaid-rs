use serde::{Serialize, Deserialize};
use super::BankTransferSweep;
///BankTransferSweepGetResponse defines the response schema for `/bank_transfer/sweep/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferSweepGetResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///BankTransferSweep describes a sweep transfer.
    pub sweep: BankTransferSweep,
}
impl std::fmt::Display for BankTransferSweepGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}