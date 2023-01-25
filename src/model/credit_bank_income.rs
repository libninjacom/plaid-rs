
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncome {
    pub bank_income_id: Option<String>,
    pub bank_income_summary: Option<CreditBankIncomeSummary>,
    pub days_requested: Option<i64>,
    pub generated_time: Option<String>,
    pub items: Option<Vec<CreditBankIncomeItem>>,
    pub warnings: Option<Vec<CreditBankIncomeWarning>>,
}
impl std::fmt::Display for CreditBankIncome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}