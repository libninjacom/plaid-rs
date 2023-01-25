
use serde::{Serialize, Deserialize};
use super::{NumbersInternationalIban, RecipientBacs};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletNumbers {
    pub bacs: Option<RecipientBacs>,
    pub international: Option<NumbersInternationalIban>,
}
impl std::fmt::Display for WalletNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}