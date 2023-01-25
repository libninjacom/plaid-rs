
use serde::{Serialize, Deserialize};
use super::{EarningsBreakdown, EarningsTotal};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Earnings {
    pub breakdown: Option<Vec<EarningsBreakdown>>,
    pub subtotals: Option<Vec<EarningsTotal>>,
    pub total: Option<EarningsTotal>,
    pub totals: Option<Vec<EarningsTotal>>,
}
impl std::fmt::Display for Earnings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}