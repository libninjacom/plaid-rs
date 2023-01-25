
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationPrecheckMilitaryInfo {
    pub branch: Option<String>,
    pub is_active_duty: Option<bool>,
}
impl std::fmt::Display for IncomeVerificationPrecheckMilitaryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}