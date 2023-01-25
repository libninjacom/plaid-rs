
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsBreakdownCanonicalDescription(pub serde_json::Value);