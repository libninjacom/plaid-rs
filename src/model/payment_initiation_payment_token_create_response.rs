use serde::{Serialize, Deserialize};
///PaymentInitiationPaymentTokenCreateResponse defines the response schema for `/payment_initiation/payment/token/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationPaymentTokenCreateResponse {
    ///A `payment_token` that can be provided to Link initialization to enter the payment initiation flow
    pub payment_token: String,
    ///The date and time at which the token will expire, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format. A `payment_token` expires after 15 minutes.
    pub payment_token_expiration_time: chrono::DateTime<chrono::Utc>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaymentInitiationPaymentTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}