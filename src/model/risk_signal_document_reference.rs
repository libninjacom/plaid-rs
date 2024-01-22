use serde::{Serialize, Deserialize};
///Object containing metadata for the document
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskSignalDocumentReference {
    ///An identifier of the document referenced by the document metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///The name of the document
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    ///Status of a document for risk signal analysis
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for RiskSignalDocumentReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}