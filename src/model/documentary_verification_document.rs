
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentaryVerificationDocument {
    pub analysis: DocumentAnalysis,
    pub attempt: f64,
    pub extracted_data: Option<PhysicalDocumentExtractedData>,
    pub images: PhysicalDocumentImages,
    pub status: String,
}
impl std::fmt::Display for DocumentaryVerificationDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}