
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRepaymentListRequest {
    pub count: Option<i64>,
    pub end_date: Option<String>,
    pub offset: Option<i64>,
    pub start_date: Option<String>,
}
impl std::fmt::Display for TransferRepaymentListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}