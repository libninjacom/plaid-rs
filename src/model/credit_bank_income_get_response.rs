
use serde::{Serialize, Deserialize};
use super::CreditBankIncome;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeGetResponse {
    pub bank_income: Option<Vec<CreditBankIncome>>,
    pub request_id: String,
}
impl std::fmt::Display for CreditBankIncomeGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}