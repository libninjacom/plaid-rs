
use serde::{Serialize, Deserialize};
use super::{PlaidError, Taxform, DocumentMetadata};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationTaxformsGetResponse {
    pub document_metadata: Vec<DocumentMetadata>,
    pub error: Option<PlaidError>,
    pub request_id: Option<String>,
    pub taxforms: Vec<Taxform>,
}
impl std::fmt::Display for IncomeVerificationTaxformsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}