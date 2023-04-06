
use serde::{Serialize, Deserialize};
use super::{
    CreditFreddieMacVerificationOfAssetsVoa24, CreditFreddieVerificationOfEmploymentVoe25,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacReportsGetResponse {
    #[serde(rename = "VOA")]
    pub voa: CreditFreddieMacVerificationOfAssetsVoa24,
    #[serde(rename = "VOE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voe: Option<CreditFreddieVerificationOfEmploymentVoe25>,
    pub request_id: String,
}
impl std::fmt::Display for CreditFreddieMacReportsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}