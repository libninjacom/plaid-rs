
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationPaymentReverseRequest {
    pub amount: Option<PaymentAmountToRefund>,
    pub idempotency_key: String,
    pub payment_id: String,
    pub reference: String,
}
impl std::fmt::Display for PaymentInitiationPaymentReverseRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}