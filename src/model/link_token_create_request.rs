
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTokenCreateRequest {
    pub access_token: Option<String>,
    pub account_filters: Option<LinkTokenAccountFilters>,
    pub additional_consented_products: Option<Vec<String>>,
    pub android_package_name: Option<String>,
    pub auth: Option<LinkTokenCreateRequestAuth>,
    pub client_name: String,
    pub country_codes: Vec<String>,
    pub deposit_switch: Option<LinkTokenCreateRequestDepositSwitch>,
    pub eu_config: Option<LinkTokenEuConfig>,
    pub identity_verification: Option<LinkTokenCreateRequestIdentityVerification>,
    pub income_verification: Option<LinkTokenCreateRequestIncomeVerification>,
    pub institution_data: Option<LinkTokenCreateInstitutionData>,
    pub institution_id: Option<String>,
    pub investments: Option<LinkTokenInvestments>,
    pub language: String,
    pub link_customization_name: Option<String>,
    pub payment_initiation: Option<LinkTokenCreateRequestPaymentInitiation>,
    pub products: Option<Vec<String>>,
    pub redirect_uri: Option<String>,
    pub transfer: Option<LinkTokenCreateRequestTransfer>,
    pub update: Option<LinkTokenCreateRequestUpdate>,
    pub user: LinkTokenCreateRequestUser,
    pub user_token: Option<String>,
    pub webhook: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}