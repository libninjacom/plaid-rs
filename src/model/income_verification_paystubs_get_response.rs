
use serde::{Serialize, Deserialize};
use super::{DocumentMetadata, Paystub, PlaidError};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationPaystubsGetResponse {
    pub document_metadata: Option<Vec<DocumentMetadata>>,
    pub error: Option<PlaidError>,
    pub paystubs: Vec<Paystub>,
    pub request_id: String,
}
impl std::fmt::Display for IncomeVerificationPaystubsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}