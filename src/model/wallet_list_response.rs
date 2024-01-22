use serde::{Serialize, Deserialize};
use super::Wallet;
///WalletListResponse defines the response schema for `/wallet/list`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletListResponse {
    ///Cursor used for fetching e-wallets created before the latest e-wallet provided in this response
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///An array of e-wallets
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wallets: Vec<Wallet>,
}
impl std::fmt::Display for WalletListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}