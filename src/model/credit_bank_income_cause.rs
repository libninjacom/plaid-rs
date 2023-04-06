
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeCause {
    pub display_message: String,
    pub error_code: String,
    pub error_message: String,
    pub error_type: String,
    pub item_id: String,
}
impl std::fmt::Display for CreditBankIncomeCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}