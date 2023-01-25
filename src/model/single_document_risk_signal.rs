
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleDocumentRiskSignal {
    pub document_reference: RiskSignalDocumentReference,
    pub risk_signals: Vec<DocumentRiskSignal>,
    pub risk_summary: DocumentRiskSummary,
}
impl std::fmt::Display for SingleDocumentRiskSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}