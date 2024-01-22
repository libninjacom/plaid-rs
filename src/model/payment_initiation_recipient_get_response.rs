use serde::{Serialize, Deserialize};
use super::PaymentInitiationRecipient;
///PaymentInitiationRecipientGetResponse defines the response schema for `/payment_initiation/recipient/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationRecipientGetResponse {
    ///PaymentInitiationRecipient defines a payment initiation recipient
    #[serde(flatten)]
    pub payment_initiation_recipient: PaymentInitiationRecipient,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
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