
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionOverride {
    pub amount: f64,
    pub currency: Option<String>,
    pub date_posted: String,
    pub date_transacted: String,
    pub description: String,
}
impl std::fmt::Display for TransactionOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}