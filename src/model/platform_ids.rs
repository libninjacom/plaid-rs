
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlatformIds {
    pub employee_id: Option<String>,
    pub payroll_id: Option<String>,
    pub position_id: Option<String>,
}
impl std::fmt::Display for PlatformIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}