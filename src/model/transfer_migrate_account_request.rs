
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferMigrateAccountRequest {
    pub account_number: String,
    pub account_type: String,
    pub routing_number: String,
    pub wire_routing_number: Option<String>,
}
impl std::fmt::Display for TransferMigrateAccountRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}