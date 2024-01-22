use serde::{Serialize, Deserialize};
use super::SimulatedTransferSweep;
///Defines the response schema for `/sandbox/transfer/sweep/simulate`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxTransferSweepSimulateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /**A sweep returned from the `/sandbox/transfer/sweep/simulate` endpoint.
Can be null if there are no transfers to include in a sweep.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sweep: Option<SimulatedTransferSweep>,
}
impl std::fmt::Display for SandboxTransferSweepSimulateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}