
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeCause {
    pub display_message: Option<String>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub error_type: Option<String>,
    pub item_id: Option<String>,
}
impl std::fmt::Display for CreditBankIncomeCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}