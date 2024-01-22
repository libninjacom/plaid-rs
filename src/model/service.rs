use serde::{Serialize, Deserialize};
use super::{Statuses, VerificationOfAsset};
///A collection of details related to a fulfillment service or product in terms of request, process and result.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Service {
    ///A collection of STATUS containers.
    #[serde(rename = "STATUSES")]
    pub statuses: Statuses,
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "VERIFICATION_OF_ASSET")]
    pub verification_of_asset: VerificationOfAsset,
}
impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}