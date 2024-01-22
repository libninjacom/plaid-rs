use serde::{Serialize, Deserialize};
///Account numbers using the International Bank Account Number and BIC/SWIFT code format.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumbersInternationalIban {
    ///The Business Identifier Code, also known as SWIFT code, for this bank account.
    pub bic: String,
    ///International Bank Account Number (IBAN).
    pub iban: String,
}
impl std::fmt::Display for NumbersInternationalIban {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}