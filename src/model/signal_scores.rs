
use serde::{Serialize, Deserialize};
use super::{BankInitiatedReturnRisk, CustomerInitiatedReturnRisk};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalScores {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_initiated_return_risk: Option<BankInitiatedReturnRisk>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_initiated_return_risk: Option<CustomerInitiatedReturnRisk>,
}
impl std::fmt::Display for SignalScores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}