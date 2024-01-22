use serde::{Serialize, Deserialize};
use super::{RiskCheckStolenIdentity, RiskCheckSyntheticIdentity};
///Result summary object capturing abuse signals related to `identity abuse`, e.g. stolen and synthetic identity fraud.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskCheckIdentityAbuseSignals {
    /**Field containing the data used in determining the outcome of the stolen identity risk check.

Contains the following fields:

`score` - A score from 0 to 100 indicating the likelihood that the user is a stolen identity.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stolen_identity: Option<RiskCheckStolenIdentity>,
    /**Field containing the data used in determining the outcome of the synthetic identity risk check.

Contains the following fields:

`score` - A score from 0 to 100 indicating the likelihood that the user is a synthetic identity.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synthetic_identity: Option<RiskCheckSyntheticIdentity>,
}
impl std::fmt::Display for RiskCheckIdentityAbuseSignals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}