use serde::{Serialize, Deserialize};
use super::PaymentInitiationPayment;
///PaymentInitiationPaymentListResponse defines the response schema for `/payment_initiation/payment/list`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationPaymentListResponse {
    ///The value that, when used as the optional `cursor` parameter to `/payment_initiation/payment/list`, will return the next unreturned payment as its first payment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<chrono::DateTime<chrono::Utc>>,
    ///An array of payments that have been created, associated with the given `client_id`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payments: Vec<PaymentInitiationPayment>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaymentInitiationPaymentListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}