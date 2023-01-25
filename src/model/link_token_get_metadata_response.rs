
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenGetMetadataResponse {
    pub account_filters: Option<AccountFiltersResponse>,
    pub client_name: Option<String>,
    pub country_codes: Vec<String>,
    pub initial_products: Vec<String>,
    pub institution_data: Option<LinkTokenCreateInstitutionData>,
    pub language: Option<String>,
    pub redirect_uri: Option<String>,
    pub webhook: Option<String>,
}
impl std::fmt::Display for LinkTokenGetMetadataResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}