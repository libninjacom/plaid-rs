
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionOverride {
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    pub date_posted: chrono::NaiveDate,
    pub date_transacted: chrono::NaiveDate,
    pub description: String,
}
impl std::fmt::Display for TransactionOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}