
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmploymentVerificationGetResponse {
    pub employments: Vec<EmploymentVerification>,
    pub request_id: String,
}
impl std::fmt::Display for EmploymentVerificationGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}