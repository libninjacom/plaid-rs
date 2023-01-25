
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayrollItem {
    pub accounts: Vec<PayrollIncomeAccountData>,
    pub institution_id: String,
    pub institution_name: String,
    pub item_id: String,
    pub payroll_income: Vec<PayrollIncomeObject>,
    pub status: Option<PayrollItemStatus>,
    pub updated_at: Option<String>,
}
impl std::fmt::Display for PayrollItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}