
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportGetResponse {
    pub report: AssetReport,
    pub request_id: String,
    pub warnings: Vec<Warning>,
}
impl std::fmt::Display for AssetReportGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}