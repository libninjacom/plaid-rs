
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferTestClockCreateResponse {
    pub request_id: String,
    pub test_clock: TransferTestClock,
}
impl std::fmt::Display for SandboxTransferTestClockCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}