
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsSearchRequestOptions {
    pub include_auth_metadata: Option<bool>,
    pub include_optional_metadata: Option<bool>,
    pub include_payment_initiation_metadata: Option<bool>,
    pub oauth: Option<bool>,
    pub payment_initiation: Option<InstitutionsSearchPaymentInitiationOptions>,
}
impl std::fmt::Display for InstitutionsSearchRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}