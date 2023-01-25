
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRepaymentReturn {
    pub amount: String,
    pub event_id: i64,
    pub iso_currency_code: String,
    pub transfer_id: String,
}
impl std::fmt::Display for TransferRepaymentReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}