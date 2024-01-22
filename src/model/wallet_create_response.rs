use serde::{Serialize, Deserialize};
///WalletCreateResponse defines the response schema for `/wallet/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletCreateResponse {}
impl std::fmt::Display for WalletCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}