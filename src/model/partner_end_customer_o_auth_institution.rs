use serde::{Serialize, Deserialize};
use super::PartnerEndCustomerOAuthInstitutionEnvironments;
///The OAuth registration information for an institution.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerEndCustomerOAuthInstitution {
    ///The date on which non-OAuth Item adds will no longer be supported for this institution, or an empty string if no such date has been set by the institution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classic_disablement_date: Option<String>,
    ///Registration statuses by environment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environments: Option<PartnerEndCustomerOAuthInstitutionEnvironments>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The date on which the end customer's application was approved by the institution, or an empty string if their application has not yet been approved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub production_enablement_date: Option<String>,
}
impl std::fmt::Display for PartnerEndCustomerOAuthInstitution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}