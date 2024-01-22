use serde::{Serialize, Deserialize};
///Defines the response schema for `/sandbox/transfer/ledger/simulate_available`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxTransferLedgerSimulateAvailableResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SandboxTransferLedgerSimulateAvailableResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}