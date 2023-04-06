
use serde::{Serialize, Deserialize};
use super::{PaymentInitiationAddress, RecipientBacs};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationRecipient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PaymentInitiationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs: Option<RecipientBacs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    pub name: String,
    pub recipient_id: String,
}
impl std::fmt::Display for PaymentInitiationRecipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}