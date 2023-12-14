
use serde::{Serialize, Deserialize};
use super::{DocumentAnalysis, PhysicalDocumentExtractedData, PhysicalDocumentImages};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentaryVerificationDocument {
    pub analysis: DocumentAnalysis,
    pub attempt: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extracted_data: Option<PhysicalDocumentExtractedData>,
    pub images: PhysicalDocumentImages,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redacted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
}
impl std::fmt::Display for DocumentaryVerificationDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}