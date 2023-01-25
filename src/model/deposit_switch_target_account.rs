
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositSwitchTargetAccount {
    pub account_name: String,
    pub account_number: String,
    pub account_subtype: String,
    pub routing_number: String,
}
impl std::fmt::Display for DepositSwitchTargetAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}