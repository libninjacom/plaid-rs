
use serde::{Serialize, Deserialize};
use super::{RiskSignalDocumentReference, DocumentRiskSignal};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MultiDocumentRiskSignal {
    pub document_references: Vec<RiskSignalDocumentReference>,
    pub risk_signals: Vec<DocumentRiskSignal>,
}
impl std::fmt::Display for MultiDocumentRiskSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}