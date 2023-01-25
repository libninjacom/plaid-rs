
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditPayStubEarnings {
    pub breakdown: Vec<PayStubEarningsBreakdown>,
    pub total: PayStubEarningsTotal,
}
impl std::fmt::Display for CreditPayStubEarnings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}