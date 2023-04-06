
use serde::{Serialize, Deserialize};
use super::PaymentInitiationPayment;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationPaymentListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<chrono::DateTime<chrono::Utc>>,
    pub payments: Vec<PaymentInitiationPayment>,
    pub request_id: String,
}
impl std::fmt::Display for PaymentInitiationPaymentListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}