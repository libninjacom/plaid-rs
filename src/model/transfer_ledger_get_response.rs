use serde::{Serialize, Deserialize};
use super::TransferLedgerBalance;
///Defines the response schema for `/transfer/ledger/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferLedgerGetResponse {
    ///Information about the balance of the ledger held with Plaid.
    pub balance: TransferLedgerBalance,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferLedgerGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}