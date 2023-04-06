
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityOverride {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sedol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker_symbol: Option<String>,
}
impl std::fmt::Display for SecurityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}