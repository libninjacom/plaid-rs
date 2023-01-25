
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferOriginatorCreateResponse {
    pub company_name: String,
    pub originator_client_id: String,
    pub request_id: String,
}
impl std::fmt::Display for TransferOriginatorCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}