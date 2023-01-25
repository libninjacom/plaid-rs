
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSessionBankIncomeResult {
    pub institution_id: Option<String>,
    pub item_id: Option<String>,
    pub status: Option<String>,
}
impl std::fmt::Display for CreditSessionBankIncomeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}