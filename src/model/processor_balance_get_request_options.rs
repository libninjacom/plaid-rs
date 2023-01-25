
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorBalanceGetRequestOptions {
    pub min_last_updated_datetime: Option<String>,
}
impl std::fmt::Display for ProcessorBalanceGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}