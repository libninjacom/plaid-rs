
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconAuditTrail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_user_id: Option<String>,
    pub source: String,
}
impl std::fmt::Display for BeaconAuditTrail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}