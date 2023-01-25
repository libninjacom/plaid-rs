
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientGetResponse {
    #[serde(flatten)]
    pub payment_initiation_recipient: PaymentInitiationRecipient,
    pub request_id: String,
}
impl std::fmt::Display for PaymentInitiationRecipientGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PaymentInitiationRecipientGetResponse {
    type Target = PaymentInitiationRecipient;
    fn deref(&self) -> &Self::Target {
        &self.payment_initiation_recipient
    }
}
impl std::ops::DerefMut for PaymentInitiationRecipientGetResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.payment_initiation_recipient
    }
}