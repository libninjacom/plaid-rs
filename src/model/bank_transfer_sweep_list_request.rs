
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferSweepListRequest {
    pub count: Option<i64>,
    pub end_time: Option<String>,
    pub origination_account_id: Option<String>,
    pub start_time: Option<String>,
}
impl std::fmt::Display for BankTransferSweepListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}