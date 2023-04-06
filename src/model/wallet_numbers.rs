
use serde::{Serialize, Deserialize};
use super::{NumbersInternationalIban, RecipientBacs};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletNumbers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs: Option<RecipientBacs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub international: Option<NumbersInternationalIban>,
}
impl std::fmt::Display for WalletNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}