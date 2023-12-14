
use serde::{Serialize, Deserialize};
use super::{TransferRecurringSchedule, TransferUserInResponse};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringTransfer {
    pub account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_class: Option<String>,
    pub amount: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub funding_account_id: String,
    pub iso_currency_code: String,
    pub network: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_origination_date: Option<chrono::NaiveDate>,
    pub origination_account_id: String,
    pub recurring_transfer_id: String,
    pub schedule: TransferRecurringSchedule,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock_id: Option<String>,
    pub transfer_ids: Vec<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub user: TransferUserInResponse,
}
impl std::fmt::Display for RecurringTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}