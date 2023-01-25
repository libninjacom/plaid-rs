
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositSwitchCreateRequest {
    pub country_code: Option<String>,
    pub options: Option<DepositSwitchCreateRequestOptions>,
    pub target_access_token: String,
    pub target_account_id: String,
}
impl std::fmt::Display for DepositSwitchCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}