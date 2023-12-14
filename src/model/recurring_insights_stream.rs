
use serde::{Serialize, Deserialize};
use super::TransactionStreamAmount;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecurringInsightsStream {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_amount: Option<TransactionStreamAmount>,
    pub average_days_apart: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    pub is_active: bool,
    pub merchant_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newest_transaction_amount: Option<TransactionStreamAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newest_transaction_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_transaction_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_finance_category_detailed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_finance_category_primary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    pub stream_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_ids: Option<Vec<String>>,
}
impl std::fmt::Display for RecurringInsightsStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}