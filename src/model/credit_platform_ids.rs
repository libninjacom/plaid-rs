
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditPlatformIds {
    pub employee_id: Option<String>,
    pub payroll_id: Option<String>,
    pub position_id: Option<String>,
}
impl std::fmt::Display for CreditPlatformIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}