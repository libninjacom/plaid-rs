
use serde::{Serialize, Deserialize};
use super::AccountBase;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorBalanceGetResponse {
    pub account: AccountBase,
    pub request_id: String,
}
impl std::fmt::Display for ProcessorBalanceGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}