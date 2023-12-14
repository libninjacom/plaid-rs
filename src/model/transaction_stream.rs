
use serde::{Serialize, Deserialize};
use super::{PersonalFinanceCategory, TransactionStreamAmount};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStream {
    pub account_id: String,
    pub average_amount: TransactionStreamAmount,
    pub category: Vec<String>,
    pub category_id: String,
    pub description: String,
    pub first_date: chrono::NaiveDate,
    pub frequency: String,
    pub is_active: bool,
    pub is_user_modified: bool,
    pub last_amount: TransactionStreamAmount,
    pub last_date: chrono::NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_user_modified_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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