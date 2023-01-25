
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationConsentPaymentExecuteResponse {
    pub payment_id: String,
    pub request_id: String,
    pub status: String,
}
impl std::fmt::Display for PaymentInitiationConsentPaymentExecuteResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}