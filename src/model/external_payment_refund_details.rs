use serde::{Serialize, Deserialize};
use super::RecipientBacs;
///Details about external payment refund
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalPaymentRefundDetails {
    ///An object containing a BACS account number and sort code. If an IBAN is not provided or if this recipient needs to accept domestic GBP-denominated payments, BACS data is required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bacs: Option<RecipientBacs>,
    ///The International Bank Account Number (IBAN) for the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    ///The name of the account holder.
    pub name: String,
}
impl std::fmt::Display for ExternalPaymentRefundDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}