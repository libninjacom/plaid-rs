
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxBankTransferSimulateRequest {
    pub bank_transfer_id: String,
    pub event_type: String,
    pub failure_reason: Option<BankTransferFailure>,
}
impl std::fmt::Display for SandboxBankTransferSimulateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}