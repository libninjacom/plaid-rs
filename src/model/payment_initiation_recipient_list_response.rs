use serde::{Serialize, Deserialize};
use super::PaymentInitiationRecipient;
///PaymentInitiationRecipientListResponse defines the response schema for `/payment_initiation/recipient/list`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationRecipientListResponse {
    ///An array of payment recipients created for Payment Initiation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipients: Vec<PaymentInitiationRecipient>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaymentInitiationRecipientListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}