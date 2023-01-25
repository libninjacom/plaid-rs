
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientCreateRequest {
    pub address: Option<PaymentInitiationAddress>,
    pub bacs: RecipientBacsNullable,
    pub iban: Option<String>,
    pub name: String,
}
impl std::fmt::Display for PaymentInitiationRecipientCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}