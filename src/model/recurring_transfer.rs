
use serde::{Serialize, Deserialize};
use super::{TransferUserInResponse, TransferRecurringSchedule};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringTransfer {
    pub account_id: String,
    pub ach_class: Option<String>,
    pub amount: String,
    pub created: String,
    pub description: String,
    pub iso_currency_code: String,
    pub network: String,
    pub next_origination_date: String,
    pub origination_account_id: String,
    pub recurring_transfer_id: String,
    pub schedule: TransferRecurringSchedule,
    pub status: String,
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