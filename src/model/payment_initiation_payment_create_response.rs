
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationPaymentCreateResponse {
    pub payment_id: String,
    pub request_id: String,
    pub status: String,
}
impl std::fmt::Display for PaymentInitiationPaymentCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}