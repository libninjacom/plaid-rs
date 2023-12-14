
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAuthorizationDecisionRationaleCode(pub serde_json::Value);