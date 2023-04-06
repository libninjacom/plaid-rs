
use serde::{Serialize, Deserialize};
use super::CreditBankIncomeCause;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankEmploymentWarning {
    pub cause: CreditBankIncomeCause,
    pub warning_code: String,
    pub warning_type: String,
}
impl std::fmt::Display for CreditBankEmploymentWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}