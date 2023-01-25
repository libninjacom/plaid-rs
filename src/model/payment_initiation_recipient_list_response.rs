
use serde::{Serialize, Deserialize};
use super::PaymentInitiationRecipient;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationRecipientListResponse {
    pub recipients: Vec<PaymentInitiationRecipient>,
    pub request_id: String,
}
impl std::fmt::Display for PaymentInitiationRecipientListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}