
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxTransferSimulateRequest {
    pub event_type: String,
    pub failure_reason: Option<TransferFailure>,
    pub transfer_id: String,
}
impl std::fmt::Display for SandboxTransferSimulateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}