
use serde::{Serialize, Deserialize};
use super::PaymentInitiationPayment;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationPaymentListResponse {
    pub next_cursor: Option<String>,
    pub payments: Vec<PaymentInitiationPayment>,
    pub request_id: String,
}
impl std::fmt::Display for PaymentInitiationPaymentListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}