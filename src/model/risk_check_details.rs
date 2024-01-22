use serde::{Serialize, Deserialize};
use super::{
    RiskCheckBehavior, RiskCheckDevice, RiskCheckEmail, RiskCheckIdentityAbuseSignals,
    RiskCheckPhone,
};
///Additional information for the `risk_check` step.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskCheckDetails {
    ///Result summary object specifying values for `behavior` attributes of risk check, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behavior: Option<RiskCheckBehavior>,
    ///Array of result summary objects specifying values for `device` attributes of risk check.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<RiskCheckDevice>,
    ///Result summary object specifying values for `email` attributes of risk check.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<RiskCheckEmail>,
    ///Result summary object capturing abuse signals related to `identity abuse`, e.g. stolen and synthetic identity fraud.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity_abuse_signals: Option<RiskCheckIdentityAbuseSignals>,
    ///Result summary object specifying values for `phone` attributes of risk check.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<RiskCheckPhone>,
    ///The status of a step in the Identity Verification process.
    pub status: String,
}
impl std::fmt::Display for RiskCheckDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}