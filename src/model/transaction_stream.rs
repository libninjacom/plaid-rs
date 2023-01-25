
use serde::{Serialize, Deserialize};
use super::{PersonalFinanceCategory, TransactionStreamAmount};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStream {
    pub account_id: String,
    pub average_amount: TransactionStreamAmount,
    pub category: Vec<String>,
    pub category_id: String,
    pub description: String,
    pub first_date: String,
    pub frequency: String,
    pub is_active: bool,
    pub last_amount: TransactionStreamAmount,
    pub last_date: String,
    pub merchant_name: Option<String>,
    pub personal_finance_category: Option<PersonalFinanceCategory>,
    pub status: String,
    pub stream_id: String,
    pub transaction_ids: Vec<String>,
}
impl std::fmt::Display for TransactionStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}