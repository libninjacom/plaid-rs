
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsSyncRequest {
    pub access_token: String,
    pub count: Option<i64>,
    pub cursor: Option<String>,
    pub options: Option<TransactionsSyncRequestOptions>,
}
impl std::fmt::Display for TransactionsSyncRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}