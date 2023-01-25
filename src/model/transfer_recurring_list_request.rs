
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRecurringListRequest {
    pub count: Option<i64>,
    pub end_time: Option<String>,
    pub offset: Option<i64>,
    pub start_time: Option<String>,
}
impl std::fmt::Display for TransferRecurringListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}