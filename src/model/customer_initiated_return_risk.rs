
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerInitiatedReturnRisk {
    pub risk_tier: i64,
    pub score: i64,
}
impl std::fmt::Display for CustomerInitiatedReturnRisk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}