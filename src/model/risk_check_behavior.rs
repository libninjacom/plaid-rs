
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskCheckBehavior {
    pub bot_detected: String,
    pub fraud_ring_detected: String,
    pub user_interactions: String,
}
impl std::fmt::Display for RiskCheckBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}