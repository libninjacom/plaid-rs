
use serde::{Serialize, Deserialize};
use super::PartnerEndCustomerOAuthInstitution;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerCustomerOAuthInstitutionsGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flowdown_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institutions: Option<Vec<PartnerEndCustomerOAuthInstitution>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub questionnaire_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl std::fmt::Display for PartnerCustomerOAuthInstitutionsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}