use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EarningsBreakdownCanonicalDescription(pub serde_json::Value);