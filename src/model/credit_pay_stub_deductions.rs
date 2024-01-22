use serde::{Serialize, Deserialize};
use super::{PayStubDeductionsBreakdown, PayStubDeductionsTotal};
///An object with the deduction information found on a pay stub.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditPayStubDeductions {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub breakdown: Vec<PayStubDeductionsBreakdown>,
    ///An object representing the total deductions for the pay period
    pub total: PayStubDeductionsTotal,
}
impl std::fmt::Display for CreditPayStubDeductions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}