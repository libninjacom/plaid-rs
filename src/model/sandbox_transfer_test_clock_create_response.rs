use serde::{Serialize, Deserialize};
use super::TransferTestClock;
///Defines the response schema for `/sandbox/transfer/test_clock/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxTransferTestClockCreateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///Defines the test clock for a transfer.
    pub test_clock: TransferTestClock,
}
impl std::fmt::Display for SandboxTransferTestClockCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}