
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskSignalDocumentReference {
    pub document_id: Option<String>,
    pub document_name: Option<String>,
}
impl std::fmt::Display for RiskSignalDocumentReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}