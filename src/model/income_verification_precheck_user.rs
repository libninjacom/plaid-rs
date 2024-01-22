use serde::{Serialize, Deserialize};
use super::SignalAddressData;
///Information about the user whose eligibility is being evaluated.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationPrecheckUser {
    ///The user's email address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    ///The user's first name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    ///Data about the components comprising an address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub home_address: Option<SignalAddressData>,
    ///The user's last name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}
impl std::fmt::Display for IncomeVerificationPrecheckUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}