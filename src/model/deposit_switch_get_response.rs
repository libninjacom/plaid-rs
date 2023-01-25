
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositSwitchGetResponse {
    pub account_has_multiple_allocations: Option<bool>,
    pub amount_allocated: Option<f64>,
    pub date_completed: Option<String>,
    pub date_created: String,
    pub deposit_switch_id: String,
    pub employer_id: Option<String>,
    pub employer_name: Option<String>,
    pub institution_id: Option<String>,
    pub institution_name: Option<String>,
    pub is_allocated_remainder: Option<bool>,
    pub percent_allocated: Option<f64>,
    pub request_id: String,
    pub state: String,
    pub switch_method: Option<serde_json::Value>,
    pub target_account_id: Option<String>,
    pub target_item_id: Option<String>,
}
impl std::fmt::Display for DepositSwitchGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}