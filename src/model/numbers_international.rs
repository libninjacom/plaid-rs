
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumbersInternational {
    pub account_id: String,
    pub bic: String,
    pub iban: String,
}
impl std::fmt::Display for NumbersInternational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}