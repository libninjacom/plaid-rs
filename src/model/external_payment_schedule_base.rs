
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalPaymentScheduleBase {
    pub adjusted_start_date: Option<String>,
    pub end_date: Option<String>,
    pub interval: Option<String>,
    pub interval_execution_day: Option<i64>,
    pub start_date: Option<String>,
}
impl std::fmt::Display for ExternalPaymentScheduleBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}