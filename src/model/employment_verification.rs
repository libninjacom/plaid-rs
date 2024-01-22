use serde::{Serialize, Deserialize};
use super::{EmployerVerification, EmploymentVerificationStatus, PlatformIds};
///An object containing proof of employment data for an individual
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmploymentVerification {
    ///An object containing employer data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employer: Option<EmployerVerification>,
    ///End of employment, if applicable. Provided in ISO 8601 format (YYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    ///An object containing a set of ids related to an employee
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform_ids: Option<PlatformIds>,
    ///Start of employment in ISO 8601 format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    ///Current employment status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EmploymentVerificationStatus>,
    ///Current title of employee.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl std::fmt::Display for EmploymentVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}