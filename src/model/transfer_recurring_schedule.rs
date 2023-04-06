
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRecurringSchedule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    pub interval_count: i64,
    pub interval_execution_day: i64,
    pub interval_unit: String,
    pub start_date: chrono::NaiveDate,
}
impl std::fmt::Display for TransferRecurringSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}