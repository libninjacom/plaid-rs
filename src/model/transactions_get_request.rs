
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsGetRequest {
    pub access_token: String,
    pub end_date: String,
    pub options: Option<TransactionsGetRequestOptions>,
    pub start_date: String,
}
impl std::fmt::Display for TransactionsGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}