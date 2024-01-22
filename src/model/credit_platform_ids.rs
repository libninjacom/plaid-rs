use serde::{Serialize, Deserialize};
///The object containing a set of ids related to an employee.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditPlatformIds {
    ///The ID of an employee as given by their employer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    ///The ID of an employee as given by their payroll.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payroll_id: Option<String>,
    ///The ID of the position of the employee.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
}
impl std::fmt::Display for CreditPlatformIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}