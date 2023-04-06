
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentTokenCreateResponse {
    pub payment_token: String,
    pub payment_token_expiration_time: chrono::DateTime<chrono::Utc>,
    pub request_id: String,
}
impl std::fmt::Display for PaymentInitiationPaymentTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}