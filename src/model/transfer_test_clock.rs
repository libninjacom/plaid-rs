
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferTestClock {
    pub test_clock_id: String,
    pub virtual_time: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for TransferTestClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}