use serde::{Serialize, Deserialize};
use super::{MultiDocumentRiskSignal, SingleDocumentRiskSignal};
///Object containing fraud risk data for a set of income documents.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentRiskSignalsObject {
    ///ID of the payroll provider account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    ///Array of risk signals computed from a set of uploaded documents and the associated documents' metadata
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub multi_document_risk_signals: Vec<MultiDocumentRiskSignal>,
    ///Array of document metadata and associated risk signals per document
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub single_document_risk_signals: Vec<SingleDocumentRiskSignal>,
}
impl std::fmt::Display for DocumentRiskSignalsObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}