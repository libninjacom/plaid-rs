use serde::{Serialize, Deserialize};
use super::WalletTransaction;
///WalletTransactionListResponse defines the response schema for `/wallet/transaction/list`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletTransactionListResponse {
    ///Cursor used for fetching transactions created before the latest transaction provided in this response
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///An array of transactions of an e-wallet, associated with the given `wallet_id`
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transactions: Vec<WalletTransaction>,
}
impl std::fmt::Display for WalletTransactionListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}