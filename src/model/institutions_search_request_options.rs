use serde::{Serialize, Deserialize};
use super::InstitutionsSearchPaymentInitiationOptions;
///An optional object to filter `/institutions/search` results.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsSearchRequestOptions {
    ///When `true`, returns metadata related to the Auth product indicating which auth methods are supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_auth_metadata: Option<bool>,
    ///When true, return the institution's homepage URL, logo and primary brand color.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_optional_metadata: Option<bool>,
    ///When `true`, returns metadata related to the Payment Initiation product indicating which payment configurations are supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_payment_initiation_metadata: Option<bool>,
    ///Limit results to institutions with or without OAuth login flows. Note that institutions will have `oauth` set to `true` if some Items associated with that institution are required to use OAuth flows; institutions in a state of migration to OAuth will have the `oauth` attribute set to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth: Option<bool>,
    ///Additional options that will be used to filter institutions by various Payment Initiation configurations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_initiation: Option<InstitutionsSearchPaymentInitiationOptions>,
}
impl std::fmt::Display for InstitutionsSearchRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}