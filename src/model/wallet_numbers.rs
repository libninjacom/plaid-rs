use serde::{Serialize, Deserialize};
use super::{NumbersInternationalIban, RecipientBacs};
///An object representing the e-wallet account numbers
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletNumbers {
    ///An object containing a BACS account number and sort code. If an IBAN is not provided or if you need to accept domestic GBP-denominated payments, BACS data is required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bacs: Option<RecipientBacs>,
    ///Account numbers using the International Bank Account Number and BIC/SWIFT code format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub international: Option<NumbersInternationalIban>,
}
impl std::fmt::Display for WalletNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}