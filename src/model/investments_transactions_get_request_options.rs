
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvestmentsTransactionsGetRequestOptions {
    pub account_ids: Option<Vec<String>>,
    pub count: Option<i64>,
    pub offset: Option<i64>,
}
impl std::fmt::Display for InvestmentsTransactionsGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}