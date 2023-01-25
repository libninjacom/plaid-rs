
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferEventListRequest {
    pub account_id: Option<String>,
    pub count: Option<i64>,
    pub end_date: Option<String>,
    pub event_types: Option<Vec<String>>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
    pub originator_client_id: Option<String>,
    pub start_date: Option<String>,
    pub sweep_id: Option<String>,
    pub transfer_id: Option<String>,
    pub transfer_type: Option<TransferEventListTransferType>,
}
impl std::fmt::Display for TransferEventListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}