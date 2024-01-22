use serde::{Serialize, Deserialize};
use super::IdentityDocumentMetadata;
///Document object with metadata of the document uploaded
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityDocument {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///In closed beta. Object representing metadata pertaining to the document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<IdentityDocumentMetadata>,
}
impl std::fmt::Display for IdentityDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}