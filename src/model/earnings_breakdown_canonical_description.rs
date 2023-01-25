
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsBreakdownCanonicalDescription(pub serde_json::Value);