
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAuthorizationGuaranteeDecision(pub serde_json::Value);