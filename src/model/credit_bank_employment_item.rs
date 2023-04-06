
use serde::{Serialize, Deserialize};
use super::{CreditBankEmployment, CreditBankIncomeAccount};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankEmploymentItem {
    pub bank_employment_accounts: Vec<CreditBankIncomeAccount>,
    pub bank_employments: Vec<CreditBankEmployment>,
    pub institution_id: String,
    pub institution_name: String,
    pub item_id: String,
    pub last_updated_time: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for CreditBankEmploymentItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}