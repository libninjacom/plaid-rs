
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationRecipient {
    pub address: Option<PaymentInitiationAddress>,
    pub bacs: RecipientBacsNullable,
    pub iban: Option<String>,
    pub name: String,
    pub recipient_id: String,
}
impl std::fmt::Display for PaymentInitiationRecipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}