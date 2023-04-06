
use serde::{Serialize, Deserialize};
use super::CreditBankIncomeCause;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeWarning {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<CreditBankIncomeCause>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning_type: Option<String>,
}
impl std::fmt::Display for CreditBankIncomeWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}