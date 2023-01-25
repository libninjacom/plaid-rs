
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxTransferTestClockAdvanceRequest {
    pub new_virtual_time: String,
    pub test_clock_id: String,
}
impl std::fmt::Display for SandboxTransferTestClockAdvanceRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}