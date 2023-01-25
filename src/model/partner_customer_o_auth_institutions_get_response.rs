
use serde::{Serialize, Deserialize};
use super::PartnerEndCustomerOAuthInstitution;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerCustomerOAuthInstitutionsGetResponse {
    pub flowdown_status: Option<String>,
    pub institutions: Option<Vec<PartnerEndCustomerOAuthInstitution>>,
    pub questionnaire_status: Option<String>,
    pub request_id: Option<String>,
}
impl std::fmt::Display for PartnerCustomerOAuthInstitutionsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}