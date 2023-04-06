
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferSweep {
    pub amount: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub id: String,
    pub iso_currency_code: String,
}
impl std::fmt::Display for BankTransferSweep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}