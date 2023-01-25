
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferEventListRequest {
    pub account_id: Option<String>,
    pub bank_transfer_id: Option<String>,
    pub bank_transfer_type: Option<BankTransferEventListBankTransferType>,
    pub count: Option<i64>,
    pub direction: Option<BankTransferEventListDirection>,
    pub end_date: Option<String>,
    pub event_types: Option<Vec<String>>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
    pub start_date: Option<String>,
}
impl std::fmt::Display for BankTransferEventListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}