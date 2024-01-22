use serde::{Serialize, Deserialize};
use super::{
    AccountBase, AddressMatchScore, EmailAddressMatchScore, NameMatchScore,
    PhoneNumberMatchScore,
};
///Identity match scores for an account
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountIdentityMatchScore {
    ///A single account at a financial institution.
    #[serde(flatten)]
    pub account_base: AccountBase,
    ///Score found by matching address provided by the API with the address on the account at the financial institution. The score can range from 0 to 100 where 100 is a perfect match and 0 is a no match. If the account contains multiple owners, the maximum match score is filled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressMatchScore>,
    ///Score found by matching email provided by the API with the email on the account at the financial institution. 100 is a perfect match and 0 is a no match. If the account contains multiple owners, the maximum match score is filled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<EmailAddressMatchScore>,
    ///Score found by matching name provided by the API with the name on the account at the financial institution. If the account contains multiple owners, the maximum match score is filled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<NameMatchScore>,
    ///Score found by matching phone number provided by the API with the phone number on the account at the financial institution. 100 is a perfect match and 0 is a no match. If the account contains multiple owners, the maximum match score is filled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
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