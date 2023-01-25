
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositSwitchAltCreateRequest {
    pub country_code: Option<String>,
    pub options: Option<DepositSwitchCreateRequestOptions>,
    pub target_account: DepositSwitchTargetAccount,
    pub target_user: DepositSwitchTargetUser,
}
impl std::fmt::Display for DepositSwitchAltCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}