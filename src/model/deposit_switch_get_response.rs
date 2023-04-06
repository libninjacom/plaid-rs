
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositSwitchGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_has_multiple_allocations: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_allocated: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_completed: Option<chrono::NaiveDate>,
    pub date_created: chrono::NaiveDate,
    pub deposit_switch_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_allocated_remainder: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_allocated: Option<f64>,
    pub request_id: String,
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_method: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_item_id: Option<String>,
}
impl std::fmt::Display for DepositSwitchGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}