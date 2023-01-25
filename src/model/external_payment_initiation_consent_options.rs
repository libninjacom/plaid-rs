
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalPaymentInitiationConsentOptions {
    pub bacs: PaymentInitiationOptionalRestrictionBacs,
    pub iban: Option<String>,
    pub request_refund_details: Option<bool>,
    pub wallet_id: Option<String>,
}
impl std::fmt::Display for ExternalPaymentInitiationConsentOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}