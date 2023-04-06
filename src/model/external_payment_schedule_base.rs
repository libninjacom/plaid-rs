
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalPaymentScheduleBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjusted_start_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_execution_day: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for ExternalPaymentScheduleBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}