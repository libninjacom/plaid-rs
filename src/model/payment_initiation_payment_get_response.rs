use serde::{Serialize, Deserialize};
use super::PaymentInitiationPayment;
///PaymentInitiationPaymentGetResponse defines the response schema for `/payment_initation/payment/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationPaymentGetResponse {
    ///PaymentInitiationPayment defines a payment initiation payment
    #[serde(flatten)]
    pub payment_initiation_payment: PaymentInitiationPayment,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl std::fmt::Display for PaymentInitiationPaymentGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PaymentInitiationPaymentGetResponse {
    type Target = PaymentInitiationPayment;
    fn deref(&self) -> &Self::Target {
        &self.payment_initiation_payment
    }
}
impl std::ops::DerefMut for PaymentInitiationPaymentGetResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.payment_initiation_payment
    }
}