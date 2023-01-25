
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmploymentVerification {
    pub employer: Option<EmployerVerification>,
    pub end_date: Option<String>,
    pub platform_ids: Option<PlatformIds>,
    pub start_date: Option<String>,
    pub status: Option<EmploymentVerificationStatus>,
    pub title: Option<String>,
}
impl std::fmt::Display for EmploymentVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}