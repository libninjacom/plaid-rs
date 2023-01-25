
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalEvaluateResponse {
    pub core_attributes: Option<SignalEvaluateCoreAttributes>,
    pub request_id: String,
    pub scores: SignalScores,
}
impl std::fmt::Display for SignalEvaluateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}