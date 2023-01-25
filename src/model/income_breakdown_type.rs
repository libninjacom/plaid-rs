
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeBreakdownType(pub serde_json::Value);