use serde::{Serialize, Deserialize};
///PaymentInitiationPaymentCreateResponse defines the response schema for `/payment_initiation/payment/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationPaymentCreateResponse {
    ///A unique ID identifying the payment
    pub payment_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /**For a payment returned by this endpoint, there is only one possible value:

`PAYMENT_STATUS_INPUT_NEEDED`: The initial phase of the payment*/
    pub status: String,
}
impl std::fmt::Display for PaymentInitiationPaymentCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}