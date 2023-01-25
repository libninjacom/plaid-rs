
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientProvidedEnhancedTransaction {
    pub amount: f64,
    pub description: String,
    pub enhancements: Enhancements,
    pub id: String,
    pub iso_currency_code: String,
}
impl std::fmt::Display for ClientProvidedEnhancedTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}