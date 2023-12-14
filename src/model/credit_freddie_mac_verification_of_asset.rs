
use serde::{Serialize, Deserialize};
use super::{
    CreditFreddieMacReportingInformation, CreditFreddieMacVerificationOfAssetResponse,
    ServiceProductFulfillment,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacVerificationOfAsset {
    #[serde(rename = "REPORTING_INFORMATION")]
    pub reporting_information: CreditFreddieMacReportingInformation,
    #[serde(rename = "SERVICE_PRODUCT_FULFILLMENT")]
    pub service_product_fulfillment: ServiceProductFulfillment,
    #[serde(rename = "VERIFICATION_OF_ASSET_RESPONSE")]
    pub verification_of_asset_response: CreditFreddieMacVerificationOfAssetResponse,
}
impl std::fmt::Display for CreditFreddieMacVerificationOfAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}