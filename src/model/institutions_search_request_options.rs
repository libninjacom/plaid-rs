
use serde::{Serialize, Deserialize};
use super::InstitutionsSearchPaymentInitiationOptions;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsSearchRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_auth_metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_optional_metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_payment_initiation_metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_initiation: Option<InstitutionsSearchPaymentInitiationOptions>,
}
impl std::fmt::Display for InstitutionsSearchRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}