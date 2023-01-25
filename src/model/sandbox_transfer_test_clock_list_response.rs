
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxTransferTestClockListResponse {
    pub request_id: String,
    pub test_clocks: Vec<TransferTestClock>,
}
impl std::fmt::Display for SandboxTransferTestClockListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}