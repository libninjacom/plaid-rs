
use serde::{Serialize, Deserialize};
use super::{EmployerVerification, EmploymentVerificationStatus, PlatformIds};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmploymentVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employer: Option<EmployerVerification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_ids: Option<PlatformIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EmploymentVerificationStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl std::fmt::Display for EmploymentVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}