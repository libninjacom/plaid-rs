
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorBalanceGetRequest {
    pub options: Option<ProcessorBalanceGetRequestOptions>,
    pub processor_token: String,
}
impl std::fmt::Display for ProcessorBalanceGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}