
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityOverride {
    pub currency: Option<String>,
    pub cusip: Option<String>,
    pub isin: Option<String>,
    pub name: Option<String>,
    pub sedol: Option<String>,
    pub ticker_symbol: Option<String>,
}
impl std::fmt::Display for SecurityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}