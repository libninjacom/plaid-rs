
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumbersAch {
    pub account: String,
    pub account_id: String,
    pub can_transfer_in: Option<bool>,
    pub can_transfer_out: Option<bool>,
    pub routing: String,
    pub wire_routing: Option<String>,
}
impl std::fmt::Display for NumbersAch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}