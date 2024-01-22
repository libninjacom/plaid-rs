use serde::{Serialize, Deserialize};
use super::{PaymentInitiationAddress, RecipientBacs};
///PaymentInitiationRecipient defines a payment initiation recipient
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationRecipient {
    ///The optional address of the payment recipient's bank account. Required by most institutions outside of the UK.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<PaymentInitiationAddress>,
    ///An object containing a BACS account number and sort code. If an IBAN is not provided or if this recipient needs to accept domestic GBP-denominated payments, BACS data is required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bacs: Option<RecipientBacs>,
    ///The International Bank Account Number (IBAN) for the recipient.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    ///The name of the recipient.
    pub name: String,
    ///The ID of the recipient.
    pub recipient_id: String,
}
impl std::fmt::Display for PaymentInitiationRecipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}