
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalPaymentOptions {
    pub bacs: PaymentInitiationOptionalRestrictionBacs,
    pub iban: Option<String>,
    pub request_refund_details: Option<bool>,
    pub scheme: Option<PaymentScheme>,
    pub wallet_id: Option<String>,
}
impl std::fmt::Display for ExternalPaymentOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}