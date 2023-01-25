
use serde::{Serialize, Deserialize};
use super::Originator;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferOriginatorListResponse {
    pub originators: Vec<Originator>,
    pub request_id: String,
}
impl std::fmt::Display for TransferOriginatorListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}