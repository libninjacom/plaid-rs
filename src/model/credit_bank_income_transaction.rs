
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeTransaction {
    pub amount: Option<f64>,
    pub check_number: Option<String>,
    pub date: Option<String>,
    pub iso_currency_code: Option<String>,
    pub name: Option<String>,
    pub original_description: Option<String>,
    pub pending: Option<bool>,
    pub transaction_id: Option<String>,
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for CreditBankIncomeTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}