
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumbersAcats {
    pub account: String,
    pub account_id: String,
    pub dtc_numbers: Vec<String>,
}
impl std::fmt::Display for NumbersAcats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}