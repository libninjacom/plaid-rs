use serde::{Serialize, Deserialize};
use super::{PayStubEarningsBreakdown, PayStubEarningsTotal};
///An object representing both a breakdown of earnings on a pay stub and the total earnings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditPayStubEarnings {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub breakdown: Vec<PayStubEarningsBreakdown>,
    ///An object representing both the current pay period and year to date amount for an earning category.
    pub total: PayStubEarningsTotal,
}
impl std::fmt::Display for CreditPayStubEarnings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}