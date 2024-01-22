use serde::{Serialize, Deserialize};
use super::{DeductionsBreakdown, DeductionsTotal, Total};
///An object with the deduction information found on a paystub.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Deductions {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub breakdown: Vec<DeductionsBreakdown>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtotals: Option<Vec<Total>>,
    ///An object representing the total deductions for the pay period
    pub total: DeductionsTotal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub totals: Option<Vec<Total>>,
}
impl std::fmt::Display for Deductions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}