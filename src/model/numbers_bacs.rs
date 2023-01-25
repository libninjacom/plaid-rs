
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumbersBacs {
    pub account: String,
    pub account_id: String,
    pub sort_code: String,
}
impl std::fmt::Display for NumbersBacs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}