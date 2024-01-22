use serde::{Serialize, Deserialize};
use super::{DocumentRiskSignal, RiskSignalDocumentReference};
///Object containing risk signals and relevant metadata for a set of uploaded documents
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MultiDocumentRiskSignal {
    ///Array of objects containing attributes that could indicate if a document is fraudulent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_references: Vec<RiskSignalDocumentReference>,
    ///Array of attributes that indicate whether or not there is fraud risk with a set of documents
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub risk_signals: Vec<DocumentRiskSignal>,
}
impl std::fmt::Display for MultiDocumentRiskSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}