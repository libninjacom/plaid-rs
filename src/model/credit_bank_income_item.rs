
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeItem {
    pub bank_income_accounts: Option<Vec<CreditBankIncomeAccount>>,
    pub bank_income_sources: Option<Vec<CreditBankIncomeSource>>,
    pub institution_id: Option<String>,
    pub institution_name: Option<String>,
    pub item_id: Option<String>,
    pub last_updated_time: Option<String>,
}
impl std::fmt::Display for CreditBankIncomeItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}