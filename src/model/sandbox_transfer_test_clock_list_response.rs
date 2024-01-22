use serde::{Serialize, Deserialize};
use super::TransferTestClock;
///Defines the response schema for `/sandbox/transfer/test_clock/list`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxTransferTestClockListResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub test_clocks: Vec<TransferTestClock>,
}
impl std::fmt::Display for SandboxTransferTestClockListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}