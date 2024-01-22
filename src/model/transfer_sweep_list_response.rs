use serde::{Serialize, Deserialize};
use super::TransferSweep;
///Defines the response schema for `/transfer/sweep/list`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferSweepListResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sweeps: Vec<TransferSweep>,
}
impl std::fmt::Display for TransferSweepListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}