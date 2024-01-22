use serde::{Serialize, Deserialize};
use super::Pay;
///An object representing employment details found on a paystub.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmploymentDetails {
    ///An object representing a monetary amount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annual_salary: Option<Pay>,
    ///Date on which the employee was hired, in the YYYY-MM-DD format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for EmploymentDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}