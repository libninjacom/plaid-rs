
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationConsentPaymentExecuteRequest {
    pub amount: PaymentAmount,
    pub consent_id: String,
    pub idempotency_key: String,
}
impl std::fmt::Display for PaymentInitiationConsentPaymentExecuteRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}