use serde::{Serialize, Deserialize};
///WalletTransactionGetResponse defines the response schema for `/wallet/transaction/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletTransactionGetResponse {}
impl std::fmt::Display for WalletTransactionGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}