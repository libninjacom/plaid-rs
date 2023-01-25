
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditDocumentMetadata {
    pub document_type: Option<String>,
    pub download_url: Option<String>,
    pub name: String,
    pub status: Option<String>,
}
impl std::fmt::Display for CreditDocumentMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}