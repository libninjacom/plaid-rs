
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentConsentPeriodicAmount {
    pub alignment: String,
    pub amount: PaymentConsentPeriodicAmountAmount,
    pub interval: String,
}
impl std::fmt::Display for PaymentConsentPeriodicAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}