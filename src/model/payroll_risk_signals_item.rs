
use serde::{Serialize, Deserialize};
use super::DocumentRiskSignalsObject;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayrollRiskSignalsItem {
    pub item_id: String,
    pub verification_risk_signals: Vec<DocumentRiskSignalsObject>,
}
impl std::fmt::Display for PayrollRiskSignalsItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}