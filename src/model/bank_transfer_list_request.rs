
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferListRequest {
    pub count: Option<i64>,
    pub direction: Option<BankTransferDirection>,
    pub end_date: Option<String>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
    pub start_date: Option<String>,
}
impl std::fmt::Display for BankTransferListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}