
use serde::{Serialize, Deserialize};
use super::{RiskCheckStolenIdentity, RiskCheckSyntheticIdentity};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskCheckIdentityAbuseSignals {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stolen_identity: Option<RiskCheckStolenIdentity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthetic_identity: Option<RiskCheckSyntheticIdentity>,
}
impl std::fmt::Display for RiskCheckIdentityAbuseSignals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}