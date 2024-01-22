use serde::{Serialize, Deserialize};
use super::{AccountFiltersResponse, LinkTokenCreateInstitutionData};
///An object specifying the arguments originally provided to the `/link/token/create` call.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenGetMetadataResponse {
    ///The `account_filters` specified in the original call to `/link/token/create`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_filters: Option<AccountFiltersResponse>,
    ///The `client_name` specified in the `/link/token/create` call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    ///The `country_codes` specified in the `/link/token/create` call.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub country_codes: Vec<String>,
    ///The `products` specified in the `/link/token/create` call.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initial_products: Vec<String>,
    ///A map containing data used to highlight institutions in Link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_data: Option<LinkTokenCreateInstitutionData>,
    ///The `language` specified in the `/link/token/create` call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    ///The `redirect_uri` specified in the `/link/token/create` call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    ///The `webhook` specified in the `/link/token/create` call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}
impl std::fmt::Display for LinkTokenGetMetadataResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}