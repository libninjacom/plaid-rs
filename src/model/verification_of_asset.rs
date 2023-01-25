
use serde::{Serialize, Deserialize};
use super::{
    ServiceProductFulfillment, VerificationOfAssetResponse, ReportingInformation,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationOfAsset {
    #[serde(rename = "REPORTING_INFORMATION")]
    pub reporting_information: ReportingInformation,
    #[serde(rename = "SERVICE_PRODUCT_FULFILLMENT")]
    pub service_product_fulfillment: ServiceProductFulfillment,
    #[serde(rename = "VERIFICATION_OF_ASSET_RESPONSE")]
    pub verification_of_asset_response: VerificationOfAssetResponse,
}
impl std::fmt::Display for VerificationOfAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}