
use serde::{Serialize, Deserialize};
use super::{
    RiskCheckBehavior, RiskCheckDevice, RiskCheckEmail, RiskCheckIdentityAbuseSignals,
    RiskCheckPhone,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskCheckDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<RiskCheckBehavior>,
    pub devices: Vec<RiskCheckDevice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<RiskCheckEmail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_abuse_signals: Option<RiskCheckIdentityAbuseSignals>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<RiskCheckPhone>,
    pub status: String,
}
impl std::fmt::Display for RiskCheckDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}