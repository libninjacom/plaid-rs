
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositSwitchCreateResponse {
    pub deposit_switch_id: String,
    pub request_id: String,
}
impl std::fmt::Display for DepositSwitchCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}