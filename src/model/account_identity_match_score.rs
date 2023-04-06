
use serde::{Serialize, Deserialize};
use super::{
    AccountBase, AddressMatchScore, EmailAddressMatchScore, NameMatchScore,
    PhoneNumberMatchScore,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountIdentityMatchScore {
    #[serde(flatten)]
    pub account_base: AccountBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressMatchScore>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<EmailAddressMatchScore>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<NameMatchScore>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<PhoneNumberMatchScore>,
}
impl std::fmt::Display for AccountIdentityMatchScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AccountIdentityMatchScore {
    type Target = AccountBase;
    fn deref(&self) -> &Self::Target {
        &self.account_base
    }
}
impl std::ops::DerefMut for AccountIdentityMatchScore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.account_base
    }
}