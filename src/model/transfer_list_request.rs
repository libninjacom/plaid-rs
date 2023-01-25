
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferListRequest {
    pub count: Option<i64>,
    pub end_date: Option<String>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
    pub originator_client_id: Option<String>,
    pub start_date: Option<String>,
}
impl std::fmt::Display for TransferListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}