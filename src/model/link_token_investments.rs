
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenInvestments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_manual_entry: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unverified_crypto_wallets: Option<bool>,
}
impl std::fmt::Display for LinkTokenInvestments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}