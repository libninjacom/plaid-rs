
use serde::{Serialize, Deserialize};
use super::SecurityOverride;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldingsOverride {
    pub cost_basis: Option<f64>,
    pub currency: String,
    pub institution_price: f64,
    pub institution_price_as_of: Option<String>,
    pub quantity: f64,
    pub security: SecurityOverride,
}
impl std::fmt::Display for HoldingsOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}