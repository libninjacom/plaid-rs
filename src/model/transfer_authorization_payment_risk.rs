
use serde::{Serialize, Deserialize};
use super::SignalWarning;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferAuthorizationPaymentRisk {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_initiated_return_score: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_initiated_return_score: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<String>,
    pub warnings: Vec<SignalWarning>,
}
impl std::fmt::Display for TransferAuthorizationPaymentRisk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}