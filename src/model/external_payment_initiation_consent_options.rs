
use serde::{Serialize, Deserialize};
use super::RecipientBacs;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalPaymentInitiationConsentOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs: Option<RecipientBacs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_refund_details: Option<bool>,
}
impl std::fmt::Display for ExternalPaymentInitiationConsentOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}