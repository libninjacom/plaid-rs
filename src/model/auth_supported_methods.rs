
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthSupportedMethods {
    pub automated_micro_deposits: bool,
    pub instant_auth: bool,
    pub instant_match: bool,
}
impl std::fmt::Display for AuthSupportedMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}