
use serde::{Serialize, Deserialize};
use super::SignalAddressData;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationPrecheckUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_address: Option<SignalAddressData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}
impl std::fmt::Display for IncomeVerificationPrecheckUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}