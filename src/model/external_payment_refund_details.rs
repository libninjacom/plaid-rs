
use serde::{Serialize, Deserialize};
use super::RecipientBacs;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalPaymentRefundDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs: Option<RecipientBacs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    pub name: String,
}
impl std::fmt::Display for ExternalPaymentRefundDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}