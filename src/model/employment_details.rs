
use serde::{Serialize, Deserialize};
use super::Pay;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmploymentDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_salary: Option<Pay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for EmploymentDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}