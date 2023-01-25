
use serde::{Serialize, Deserialize};
use super::{RecipientBacs, PaymentScheme};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalPaymentOptions {
    pub bacs: Option<RecipientBacs>,
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