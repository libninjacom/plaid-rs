
use serde::{Serialize, Deserialize};
use super::{
    CreditFreddieMacReportingInformationVoa24,
    CreditFreddieMacVerificationOfAssetResponseVoe25, ServiceProductFulfillment,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacVerificationOfAssetVoe25 {
    #[serde(rename = "REPORTING_INFORMATION")]
    pub reporting_information: CreditFreddieMacReportingInformationVoa24,
    #[serde(rename = "SERVICE_PRODUCT_FULFILLMENT")]
    pub service_product_fulfillment: ServiceProductFulfillment,
    #[serde(rename = "VERIFICATION_OF_ASSET_RESPONSE")]
    pub verification_of_asset_response: CreditFreddieMacVerificationOfAssetResponseVoe25,
}
impl std::fmt::Display for CreditFreddieMacVerificationOfAssetVoe25 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}