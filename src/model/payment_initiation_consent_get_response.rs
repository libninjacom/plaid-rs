use serde::{Serialize, Deserialize};
///PaymentInitiationConsentGetResponse defines the response schema for `/payment_initation/consent/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationConsentGetResponse {}
impl std::fmt::Display for PaymentInitiationConsentGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}