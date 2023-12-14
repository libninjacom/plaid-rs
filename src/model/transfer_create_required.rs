
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferCreateRequired {
    pub access_token: String,
    pub account_id: String,
    pub authorization_id: String,
    pub description: String,
}
impl std::fmt::Display for TransferCreateRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}