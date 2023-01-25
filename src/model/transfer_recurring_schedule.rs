
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRecurringSchedule {
    pub end_date: Option<String>,
    pub interval_count: i64,
    pub interval_execution_day: i64,
    pub interval_unit: String,
    pub start_date: String,
}
impl std::fmt::Display for TransferRecurringSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}