
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub beginning: chrono::NaiveDate,
    pub ending: chrono::NaiveDate,
}
impl std::fmt::Display for DateRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}