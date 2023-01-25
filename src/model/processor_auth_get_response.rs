
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorAuthGetResponse {
    pub account: AccountBase,
    pub numbers: ProcessorNumber,
    pub request_id: String,
}
impl std::fmt::Display for ProcessorAuthGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}