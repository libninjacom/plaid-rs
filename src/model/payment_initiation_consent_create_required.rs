
use serde::{Serialize, Deserialize};
use super::PaymentInitiationConsentConstraints;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationConsentCreateRequired {
    pub constraints: PaymentInitiationConsentConstraints,
    pub recipient_id: String,
    pub reference: String,
    pub scopes: Vec<String>,
}
impl std::fmt::Display for PaymentInitiationConsentCreateRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}