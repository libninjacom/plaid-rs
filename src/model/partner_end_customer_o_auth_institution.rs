
use serde::{Serialize, Deserialize};
use super::PartnerEndCustomerOAuthInstitutionEnvironments;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerEndCustomerOAuthInstitution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic_disablement_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<PartnerEndCustomerOAuthInstitutionEnvironments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_enablement_date: Option<String>,
}
impl std::fmt::Display for PartnerEndCustomerOAuthInstitution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}