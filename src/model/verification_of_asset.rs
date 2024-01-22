use serde::{Serialize, Deserialize};
use super::{
    ReportingInformation, ServiceProductFulfillment, VerificationOfAssetResponse,
};
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VerificationOfAsset {
    ///Information about an report identifier and a report name.
    #[serde(rename = "REPORTING_INFORMATION")]
    pub reporting_information: ReportingInformation,
    ///A collection of details related to a fulfillment service or product in terms of request, process and result.
    #[serde(rename = "SERVICE_PRODUCT_FULFILLMENT")]
    pub service_product_fulfillment: ServiceProductFulfillment,
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "VERIFICATION_OF_ASSET_RESPONSE")]
    pub verification_of_asset_response: VerificationOfAssetResponse,
}
impl std::fmt::Display for VerificationOfAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}