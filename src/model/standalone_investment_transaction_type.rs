
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StandaloneInvestmentTransactionType {
    pub buy: String,
    pub cancel: String,
    pub cash: String,
    pub fee: String,
    pub sell: String,
    pub transfer: String,
}
impl std::fmt::Display for StandaloneInvestmentTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}