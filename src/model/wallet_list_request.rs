
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletListRequest {
    pub count: Option<i64>,
    pub cursor: Option<String>,
    pub iso_currency_code: Option<String>,
}
impl std::fmt::Display for WalletListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}