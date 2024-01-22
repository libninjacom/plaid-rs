use serde::{Serialize, Deserialize};
///A date range with a start and end date
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DateRange {
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    pub beginning: chrono::NaiveDate,
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    pub ending: chrono::NaiveDate,
}
impl std::fmt::Display for DateRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}