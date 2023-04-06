
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRepayment {
    pub amount: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub iso_currency_code: String,
    pub repayment_id: String,
}
impl std::fmt::Display for TransferRepayment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}