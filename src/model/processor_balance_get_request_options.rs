
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorBalanceGetRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_last_updated_datetime: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for ProcessorBalanceGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}