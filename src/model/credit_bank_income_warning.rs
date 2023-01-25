
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeWarning {
    pub cause: Option<CreditBankIncomeCause>,
    pub warning_code: Option<String>,
    pub warning_type: Option<String>,
}
impl std::fmt::Display for CreditBankIncomeWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}