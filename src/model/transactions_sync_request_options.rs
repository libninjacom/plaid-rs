
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsSyncRequestOptions {
    pub include_logo_and_counterparty_beta: Option<bool>,
    pub include_original_description: Option<bool>,
    pub include_personal_finance_category: Option<bool>,
}
impl std::fmt::Display for TransactionsSyncRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}