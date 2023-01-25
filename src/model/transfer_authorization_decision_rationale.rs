
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferAuthorizationDecisionRationale {
    pub code: String,
    pub description: String,
}
impl std::fmt::Display for TransferAuthorizationDecisionRationale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}