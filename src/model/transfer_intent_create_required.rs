
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferIntentCreateRequired {
    pub amount: String,
    pub description: String,
    pub mode: String,
    pub user: TransferUserInRequest,
}
impl std::fmt::Display for TransferIntentCreateRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}