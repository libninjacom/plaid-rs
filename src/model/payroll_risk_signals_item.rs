use serde::{Serialize, Deserialize};
use super::DocumentRiskSignalsObject;
///Object containing fraud risk data pertaining to the Item linked as part of the verification.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayrollRiskSignalsItem {
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    ///Array of payroll income document authenticity data retrieved for each of the user's accounts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verification_risk_signals: Vec<DocumentRiskSignalsObject>,
}
impl std::fmt::Display for PayrollRiskSignalsItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}