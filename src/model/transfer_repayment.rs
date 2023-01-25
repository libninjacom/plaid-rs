
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRepayment {
    pub amount: String,
    pub created: String,
    pub iso_currency_code: String,
    pub repayment_id: String,
}
impl std::fmt::Display for TransferRepayment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}