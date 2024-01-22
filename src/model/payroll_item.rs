use serde::{Serialize, Deserialize};
use super::{PayrollIncomeAccountData, PayrollIncomeObject, PayrollItemStatus};
///An object containing information about the payroll item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayrollItem {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<PayrollIncomeAccountData>,
    ///The unique identifier of the institution associated with the Item.
    pub institution_id: String,
    ///The name of the institution associated with the Item.
    pub institution_name: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payroll_income: Vec<PayrollIncomeObject>,
    ///Details about the status of the payroll item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PayrollItemStatus>,
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DDTHH:mm:ssZ) indicating the last time that the Item was updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for PayrollItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}