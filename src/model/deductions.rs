
use serde::{Serialize, Deserialize};
use super::{DeductionsBreakdown, DeductionsTotal, Total};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deductions {
    pub breakdown: Vec<DeductionsBreakdown>,
    pub subtotals: Option<Vec<Total>>,
    pub total: DeductionsTotal,
    pub totals: Option<Vec<Total>>,
}
impl std::fmt::Display for Deductions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}