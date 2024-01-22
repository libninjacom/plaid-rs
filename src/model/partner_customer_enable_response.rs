use serde::{Serialize, Deserialize};
///Response schema for `/partner/customer/enable`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerCustomerEnableResponse {
    ///The end customer's secret key for the Production environment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub production_secret: Option<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl std::fmt::Display for PartnerCustomerEnableResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}