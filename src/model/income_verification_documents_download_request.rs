
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationDocumentsDownloadRequest {
    pub access_token: Option<String>,
    pub document_id: Option<String>,
    pub income_verification_id: Option<String>,
}
impl std::fmt::Display for IncomeVerificationDocumentsDownloadRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}