
use serde::{Serialize, Deserialize};
use super::Cause;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseReportWarning {
    pub cause: Cause,
    pub warning_code: String,
    pub warning_type: String,
}
impl std::fmt::Display for BaseReportWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}