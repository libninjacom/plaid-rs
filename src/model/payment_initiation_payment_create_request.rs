
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentCreateRequest {
    pub amount: PaymentAmount,
    pub options: Option<ExternalPaymentOptions>,
    pub recipient_id: String,
    pub reference: String,
    pub schedule: Option<ExternalPaymentScheduleRequest>,
}
impl std::fmt::Display for PaymentInitiationPaymentCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}