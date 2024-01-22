use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferAuthorizationDecisionRationaleCode(pub serde_json::Value);