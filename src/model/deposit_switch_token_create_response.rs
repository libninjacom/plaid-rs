
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositSwitchTokenCreateResponse {
    pub deposit_switch_token: String,
    pub deposit_switch_token_expiration_time: String,
    pub request_id: String,
}
impl std::fmt::Display for DepositSwitchTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}