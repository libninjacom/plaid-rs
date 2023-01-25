
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferCapabilitiesGetRequest {
    pub access_token: Option<String>,
    pub account_id: Option<String>,
    pub payment_profile_token: Option<String>,
}
impl std::fmt::Display for TransferCapabilitiesGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}