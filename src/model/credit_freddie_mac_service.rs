use serde::{Serialize, Deserialize};
use super::{CreditFreddieMacVerificationOfAsset, Statuses};
///A collection of details related to a fulfillment service or product in terms of request, process and result.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacService {
    ///A collection of STATUS containers.
    #[serde(rename = "STATUSES")]
    pub statuses: Statuses,
    #[serde(rename = "VERIFICATION_OF_ASSET")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verification_of_asset: Vec<CreditFreddieMacVerificationOfAsset>,
}
impl std::fmt::Display for CreditFreddieMacService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}