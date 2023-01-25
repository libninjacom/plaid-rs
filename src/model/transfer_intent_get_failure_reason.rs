
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferIntentGetFailureReason {
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub error_type: Option<String>,
}
impl std::fmt::Display for TransferIntentGetFailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}