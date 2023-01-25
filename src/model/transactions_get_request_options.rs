
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsGetRequestOptions {
    pub account_ids: Option<Vec<String>>,
    pub count: Option<i64>,
    pub include_logo_and_counterparty_beta: Option<bool>,
    pub include_original_description: Option<bool>,
    pub include_personal_finance_category: Option<bool>,
    pub include_personal_finance_category_beta: Option<bool>,
    pub offset: Option<i64>,
}
impl std::fmt::Display for TransactionsGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}