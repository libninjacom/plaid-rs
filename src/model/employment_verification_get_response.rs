use serde::{Serialize, Deserialize};
use super::EmploymentVerification;
///EmploymentVerificationGetResponse defines the response schema for `/employment/verification/get`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmploymentVerificationGetResponse {
    ///A list of employment verification summaries.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub employments: Vec<EmploymentVerification>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for EmploymentVerificationGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}