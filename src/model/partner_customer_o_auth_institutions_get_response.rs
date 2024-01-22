use serde::{Serialize, Deserialize};
use super::PartnerEndCustomerOAuthInstitution;
///Response schema for `/partner/customer/oauth_institutions/get`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerCustomerOAuthInstitutionsGetResponse {
    ///The status of the addendum to the Plaid MSA ("flowdown") for the end customer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flowdown_status: Option<String>,
    ///The OAuth institutions with which the end customer's application is being registered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institutions: Option<Vec<PartnerEndCustomerOAuthInstitution>>,
    ///The status of the end customer's security questionnaire.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub questionnaire_status: Option<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl std::fmt::Display for PartnerCustomerOAuthInstitutionsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}