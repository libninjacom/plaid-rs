use serde::{Serialize, Deserialize};
use super::DepositSwitchAddressData;
///The deposit switch target user
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositSwitchTargetUser {
    ///The user's address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<DepositSwitchAddressData>,
    ///The email address of the user.
    pub email: String,
    ///The family name (last name) of the user.
    pub family_name: String,
    ///The given name (first name) of the user.
    pub given_name: String,
    ///The phone number of the user. The endpoint can accept a variety of phone number formats, including E.164.
    pub phone: String,
    ///The taxpayer ID of the user, generally their SSN, EIN, or TIN.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_payer_id: Option<String>,
}
impl std::fmt::Display for DepositSwitchTargetUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}