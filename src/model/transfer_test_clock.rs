use serde::{Serialize, Deserialize};
///Defines the test clock for a transfer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferTestClock {
    ///Plaid’s unique identifier for a test clock.
    pub test_clock_id: String,
    ///The virtual timestamp on the test clock. This will be of the form `2006-01-02T15:04:05Z`.
    pub virtual_time: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for TransferTestClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}