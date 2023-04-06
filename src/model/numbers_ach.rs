
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumbersAch {
    pub account: String,
    pub account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_transfer_in: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_transfer_out: Option<bool>,
    pub routing: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wire_routing: Option<String>,
}
impl std::fmt::Display for NumbersAch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}