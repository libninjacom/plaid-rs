use serde::{Serialize, Deserialize};
/**Field containing the data used in determining the outcome of the stolen identity risk check.

Contains the following fields:

`score` - A score from 0 to 100 indicating the likelihood that the user is a stolen identity.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskCheckStolenIdentity {
    ///A score from 0 to 100 indicating the likelihood that the user is a stolen identity.
    pub score: i64,
}
impl std::fmt::Display for RiskCheckStolenIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}