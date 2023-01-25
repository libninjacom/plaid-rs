
use serde::{Serialize, Deserialize};
use super::BankTransferSweep;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferSweepListResponse {
    pub request_id: String,
    pub sweeps: Vec<BankTransferSweep>,
}
impl std::fmt::Display for BankTransferSweepListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}