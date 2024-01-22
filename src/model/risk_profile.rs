use serde::{Serialize, Deserialize};
///Details about the transaction result after evaluated by the requested risk profile. If a `risk_profile_key` is not provided, this field will be omitted. This feature is currently in closed beta; to request access, contact your account manager.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskProfile {
    ///The name of the risk profile used for this transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    ///The name of the evaluated outcome for this transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
}
impl std::fmt::Display for RiskProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}