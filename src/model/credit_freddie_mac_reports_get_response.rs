
use serde::{Serialize, Deserialize};
use super::{
    CreditFreddieVerificationOfEmploymentVoe25, CreditFreddieMacVerificationOfAssetsVoa24,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacReportsGetResponse {
    #[serde(rename = "VOA")]
    pub voa: CreditFreddieMacVerificationOfAssetsVoa24,
    #[serde(rename = "VOE")]
    pub voe: CreditFreddieVerificationOfEmploymentVoe25,
    pub request_id: String,
}
impl std::fmt::Display for CreditFreddieMacReportsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}