
use serde::{Serialize, Deserialize};
use super::RecipientBacs;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalPaymentRefundDetails {
    pub bacs: Option<RecipientBacs>,
    pub iban: Option<String>,
    pub name: String,
}
impl std::fmt::Display for ExternalPaymentRefundDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}