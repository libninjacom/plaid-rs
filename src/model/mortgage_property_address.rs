
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MortgagePropertyAddress {
    pub city: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub region: Option<String>,
    pub street: Option<String>,
}
impl std::fmt::Display for MortgagePropertyAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}