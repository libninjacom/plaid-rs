
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsRecurringGetRequest {
    pub access_token: String,
    pub account_ids: Vec<String>,
    pub options: Option<TransactionsRecurringGetRequestOptions>,
}
impl std::fmt::Display for TransactionsRecurringGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}