
use serde::{Serialize, Deserialize};
use super::{PaymentScheme, RecipientBacs};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalPaymentOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs: Option<RecipientBacs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_refund_details: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<PaymentScheme>,
}
impl std::fmt::Display for ExternalPaymentOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}