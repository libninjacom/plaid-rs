
use serde::{Serialize, Deserialize};
use super::{EarningsBreakdown, EarningsTotal};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Earnings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<Vec<EarningsBreakdown>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotals: Option<Vec<EarningsTotal>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<EarningsTotal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totals: Option<Vec<EarningsTotal>>,
}
impl std::fmt::Display for Earnings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}