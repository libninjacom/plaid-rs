
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationConsentCreateResponse {
    pub consent_id: String,
    pub request_id: String,
    pub status: String,
}
impl std::fmt::Display for PaymentInitiationConsentCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}