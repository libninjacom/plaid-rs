
use serde::{Serialize, Deserialize};
use super::SimulatedTransferSweep;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxTransferSweepSimulateResponse {
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sweep: Option<SimulatedTransferSweep>,
}
impl std::fmt::Display for SandboxTransferSweepSimulateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}