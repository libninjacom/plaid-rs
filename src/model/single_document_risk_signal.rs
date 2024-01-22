use serde::{Serialize, Deserialize};
use super::{DocumentRiskSignal, DocumentRiskSummary, RiskSignalDocumentReference};
///Object containing all risk signals and relevant metadata for a single document
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SingleDocumentRiskSignal {
    ///Object containing metadata for the document
    pub document_reference: RiskSignalDocumentReference,
    ///Array of attributes that indicate whether or not there is fraud risk with a document
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub risk_signals: Vec<DocumentRiskSignal>,
    ///A summary across all risk signals associated with a document
    pub risk_summary: DocumentRiskSummary,
}
impl std::fmt::Display for SingleDocumentRiskSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}