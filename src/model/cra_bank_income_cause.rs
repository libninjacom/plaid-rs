
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBankIncomeCause {
    pub display_message: String,
    pub error_code: String,
    pub error_message: String,
    pub error_type: String,
}
impl std::fmt::Display for CraBankIncomeCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}