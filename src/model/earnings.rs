use serde::{Serialize, Deserialize};
use super::{EarningsBreakdown, EarningsTotal};
///An object representing both a breakdown of earnings on a paystub and the total earnings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Earnings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<Vec<EarningsBreakdown>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtotals: Option<Vec<EarningsTotal>>,
    ///An object representing both the current pay period and year to date amount for an earning category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<EarningsTotal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub totals: Option<Vec<EarningsTotal>>,
}
impl std::fmt::Display for Earnings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}