
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumbersEft {
    pub account: String,
    pub account_id: String,
    pub branch: String,
    pub institution: String,
}
impl std::fmt::Display for NumbersEft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}