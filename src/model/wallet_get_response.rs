use serde::{Serialize, Deserialize};
///WalletGetResponse defines the response schema for `/wallet/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletGetResponse {}
impl std::fmt::Display for WalletGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}