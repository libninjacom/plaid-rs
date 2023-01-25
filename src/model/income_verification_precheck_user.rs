
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationPrecheckUser {
    pub email_address: Option<String>,
    pub first_name: Option<String>,
    pub home_address: Option<SignalAddressData>,
    pub last_name: Option<String>,
}
impl std::fmt::Display for IncomeVerificationPrecheckUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}