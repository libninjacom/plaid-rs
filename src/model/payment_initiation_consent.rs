
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationConsent {
    pub consent_id: String,
    pub constraints: PaymentInitiationConsentConstraints,
    pub created_at: String,
    pub recipient_id: String,
    pub reference: String,
    pub scopes: Vec<String>,
    pub status: String,
}
impl std::fmt::Display for PaymentInitiationConsent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}