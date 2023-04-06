
use serde::{Serialize, Deserialize};
use super::{AccountFiltersResponse, LinkTokenCreateInstitutionData};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenGetMetadataResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_filters: Option<AccountFiltersResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    pub country_codes: Vec<String>,
    pub initial_products: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_data: Option<LinkTokenCreateInstitutionData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}
impl std::fmt::Display for LinkTokenGetMetadataResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}