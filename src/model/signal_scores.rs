
use serde::{Serialize, Deserialize};
use super::{BankInitiatedReturnRisk, CustomerInitiatedReturnRisk};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalScores {
    pub bank_initiated_return_risk: Option<BankInitiatedReturnRisk>,
    pub customer_initiated_return_risk: Option<CustomerInitiatedReturnRisk>,
}
impl std::fmt::Display for SignalScores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}