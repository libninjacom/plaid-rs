use serde::{Serialize, Deserialize};
use super::{CreditEmployerVerification, CreditPlatformIds};
///The object containing proof of employment data for an individual.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditEmploymentVerification {
    ///ID of the payroll provider account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /**The type of employment for the individual.
`"FULL_TIME"`: A full-time employee.
`"PART_TIME"`: A part-time employee.
`"CONTRACTOR"`: An employee typically hired externally through a contracting group.
`"TEMPORARY"`: A temporary employee.
`"OTHER"`: The employee type is not one of the above defined types.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<String>,
    ///An object containing employer data.
    pub employer: CreditEmployerVerification,
    ///End of employment, if applicable. Provided in ISO 8601 format (YYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    ///The date of the employee's most recent paystub in ISO 8601 format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_paystub_date: Option<chrono::NaiveDate>,
    ///The object containing a set of ids related to an employee.
    pub platform_ids: CreditPlatformIds,
    ///Start of employment in ISO 8601 format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    ///Current employment status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    ///Current title of employee.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl std::fmt::Display for CreditEmploymentVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}