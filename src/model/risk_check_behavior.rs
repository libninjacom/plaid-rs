use serde::{Serialize, Deserialize};
///Result summary object specifying values for `behavior` attributes of risk check, when available.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskCheckBehavior {
    /**Field describing the outcome of a bot detection behavior risk check.

`yes` indicates that automated activity was detected.

`no` indicates that automated activity was not detected.

`no_data` indicates there was not enough information available to give an accurate signal.*/
    pub bot_detected: String,
    /**Field describing the outcome of a fraud ring behavior risk check.

`yes` indicates that fraud ring activity was detected.

`no` indicates that fraud ring activity was not detected.

`no_data` indicates there was not enough information available to give an accurate signal.*/
    pub fraud_ring_detected: String,
    /**Field describing the overall user interaction signals of a behavior risk check. This value represents how familiar the user is with the personal data they provide, based on a number of signals that are collected during their session.

`genuine` indicates the user has high familiarity with the data they are providing, and that fraud is unlikely.

`neutral` indicates some signals are present in between `risky` and `genuine`, but there are not enough clear signals to determine an outcome.

`risky` indicates the user has low familiarity with the data they are providing, and that fraud is likely.

`no_data` indicates there is not sufficient information to give an accurate signal.*/
    pub user_interactions: String,
}
impl std::fmt::Display for RiskCheckBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}