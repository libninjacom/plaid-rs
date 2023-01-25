
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxTransferTestClockListRequest {
    pub count: Option<i64>,
    pub end_virtual_time: Option<String>,
    pub offset: Option<i64>,
    pub start_virtual_time: Option<String>,
}
impl std::fmt::Display for SandboxTransferTestClockListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}