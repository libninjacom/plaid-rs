use serde::{Serialize, Deserialize};
///In closed beta. Object representing metadata pertaining to the document.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityDocumentMetadata {
    ///Boolean field indicating if the uploaded document's account number matches the account number we have on file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_account_number_match: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    ///The name of the document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i64>,
    /**The processing status of the document.

`PROCESSING_COMPLETE`: The document was successfully processed.

`DOCUMENT_ERROR`: The document could not be processed. Possible causes include: The document was an unacceptable document type such as an offer letter or bank statement, the document image was cropped or blurry, or the document was corrupted.

`UNKNOWN` or `null`: An internal error occurred. If this happens repeatedly, contact support or your Plaid account manager.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for IdentityDocumentMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}