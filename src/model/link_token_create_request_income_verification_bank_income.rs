
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestIncomeVerificationBankIncome {
    pub days_requested: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_multiple_items: Option<bool>,
}
impl std::fmt::Display for LinkTokenCreateRequestIncomeVerificationBankIncome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}