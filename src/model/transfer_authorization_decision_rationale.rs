
use serde::{Serialize, Deserialize};
use super::TransferAuthorizationDecisionRationaleCode;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAuthorizationDecisionRationale {
    pub code: TransferAuthorizationDecisionRationaleCode,
    pub description: String,
}
impl std::fmt::Display for TransferAuthorizationDecisionRationale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}