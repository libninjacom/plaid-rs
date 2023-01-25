
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditPayStubDeductions {
    pub breakdown: Vec<PayStubDeductionsBreakdown>,
    pub total: PayStubDeductionsTotal,
}
impl std::fmt::Display for CreditPayStubDeductions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}