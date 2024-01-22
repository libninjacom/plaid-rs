use serde::{Serialize, Deserialize};
use super::{
    CreditFreddieMacReportingInformation, CreditFreddieMacVerificationOfAssetResponse,
    ServiceProductFulfillment,
};
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacVerificationOfAsset {
    ///Information about an report identifier and a report name.
    #[serde(rename = "REPORTING_INFORMATION")]
    pub reporting_information: CreditFreddieMacReportingInformation,
    ///A collection of details related to a fulfillment service or product in terms of request, process and result.
    #[serde(rename = "SERVICE_PRODUCT_FULFILLMENT")]
    pub service_product_fulfillment: ServiceProductFulfillment,
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "VERIFICATION_OF_ASSET_RESPONSE")]
    pub verification_of_asset_response: CreditFreddieMacVerificationOfAssetResponse,
}
impl std::fmt::Display for CreditFreddieMacVerificationOfAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}