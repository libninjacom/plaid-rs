use serde::{Serialize, Deserialize};
use super::{DocumentAnalysis, PhysicalDocumentExtractedData, PhysicalDocumentImages};
///Images, extracted data, and analysis from a user's identity document
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentaryVerificationDocument {
    ///High level descriptions of how the associated document was processed. If a document fails verification, the details in the `analysis` object should help clarify why the document was rejected.
    pub analysis: DocumentAnalysis,
    ///The `attempt` field begins with 1 and increments with each subsequent document upload.
    pub attempt: i64,
    ///Data extracted from a user-submitted document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extracted_data: Option<PhysicalDocumentExtractedData>,
    ///URLs for downloading original and cropped images for this document submission. The URLs are designed to only allow downloading, not hot linking, so the URL will only serve the document image for 60 seconds before expiring. The expiration time is 60 seconds after the `GET` request for the associated Identity Verification attempt. A new expiring URL is generated with each request, so you can always rerequest the Identity Verification attempt if one of your URLs expires.
    pub images: PhysicalDocumentImages,
    ///An ISO8601 formatted timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redacted_at: Option<chrono::DateTime<chrono::Utc>>,
    ///An outcome status for this specific document submission. Distinct from the overall `documentary_verification.status` that summarizes the verification outcome from one or more documents.
    pub status: String,
}
impl std::fmt::Display for DocumentaryVerificationDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}