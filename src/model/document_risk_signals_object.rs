
use serde::{Serialize, Deserialize};
use super::{SingleDocumentRiskSignal, MultiDocumentRiskSignal};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentRiskSignalsObject {
    pub account_id: Option<String>,
    pub multi_document_risk_signals: Vec<MultiDocumentRiskSignal>,
    pub single_document_risk_signals: Vec<SingleDocumentRiskSignal>,
}
impl std::fmt::Display for DocumentRiskSignalsObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}