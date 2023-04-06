
use serde::{Serialize, Deserialize};
use super::{PayrollIncomeAccountData, PayrollIncomeObject, PayrollItemStatus};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayrollItem {
    pub accounts: Vec<PayrollIncomeAccountData>,
    pub institution_id: String,
    pub institution_name: String,
    pub item_id: String,
    pub payroll_income: Vec<PayrollIncomeObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PayrollItemStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for PayrollItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}