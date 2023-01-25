
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalReturnReportRequest {
    pub client_transaction_id: String,
    pub return_code: String,
    pub returned_at: Option<String>,
}
impl std::fmt::Display for SignalReturnReportRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}